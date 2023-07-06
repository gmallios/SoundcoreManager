use std::cell::RefCell;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use soundcore_lib::api::SoundcoreDevices;
use soundcore_lib::bt::ble::BLEConnectionRegistry;
use soundcore_lib::devices::a3951::device;
use soundcore_lib::{
    api::{
        DeviceDescriptor, DeviceRegistry, EQValues, SoundMode, SoundcoreDevice,
        SoundcoreDeviceState,
    },
    device_registry::{create_soundcore_device_registry, SoundcoreDeviceRegistry},
};
use tokio::sync::{mpsc, Mutex, RwLock};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum SoundcoreRequestMessage {
    GetDescriptors,                 /* Request new descriptors / Scan for new devices */
    OpenDevice(DescriptorResponse), /* "Opens" the device with the name and mac address */
    CloseDevice,                    /* "Closes" the device with the index */
    RequestNewState,                /* Request new state for the device give the index */
    SetSoundMode(SoundMode),        /* Set ANC for the device given the index */
    SetEQ(EQValues),                /* Set EQ for the device given the index */
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum SoundcoreResponseMessage {
    NewState(SoundcoreDeviceState),
    Descriptors(Vec<DescriptorResponse>),
    CreationSuccess,
    //TODO: Move to error enum
    Error(String),
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptorResponse {
    pub name: String,
    pub mac_address: String,
    pub model: String,
}

struct SoundcoreProcessState<RegistryType>
where
    RegistryType: BLEConnectionRegistry + Send + Sync + 'static,
{
    device: Option<Arc<SoundcoreDevices<RegistryType::ConnType>>>,
    soundcore_registry: SoundcoreDeviceRegistry<RegistryType>,
}

impl<'a, RegistryType> SoundcoreProcessState<RegistryType>
where
    RegistryType: BLEConnectionRegistry + Send + Sync + 'static,
{
    fn new(registry: RegistryType) -> Self {
        Self {
            device: None,
            soundcore_registry: SoundcoreDeviceRegistry::new(registry),
        }
    }

    async fn new_device(&mut self, desc: &DescriptorResponse) -> Result<(), ()> {
        let device = self
            .soundcore_registry
            .device(&desc.name, &desc.mac_address)
            .await;
        match device {
            Ok(device) => match device {
                Some(device) => {
                    self.device = Some(device);
                    Ok(())
                }
                None => Err(()),
            },
            Err(e) => {
                println!("Error: {}", e);
                Err(())
            }
        }
    }

    async fn get_device(&self) -> Option<&dyn SoundcoreDevice<RegistryType::ConnType>> {
        match &self.device {
            Some(device) => match device.to_device() {
                Some(device) => Some(device),
                None => panic!("Instanciated device should have a corresponding implementation"),
            },
            None => None,
        }
    }
}

pub async fn soundcore_process(
    mut rx: mpsc::Receiver<SoundcoreRequestMessage>,
    tx: mpsc::Sender<SoundcoreResponseMessage>,
) {
    let soundcore_registry = create_soundcore_device_registry().await;
    let mut state = SoundcoreProcessState::new(soundcore_registry);

    tokio::spawn(async move {
        while let Some(input) = rx.recv().await {
            match input {
                SoundcoreRequestMessage::GetDescriptors => {
                    match state.soundcore_registry.descriptors().await {
                        Ok(descriptors) => {
                            let descriptors: Vec<DescriptorResponse> = descriptors
                                .into_iter()
                                .map(|desc| DescriptorResponse {
                                    name: desc.name().to_string(),
                                    mac_address: desc.mac_address().to_string(),
                                    model: desc.model_id().unwrap().to_string(),
                                })
                                .collect();
                            let _ = tx
                                .send(SoundcoreResponseMessage::Descriptors(descriptors))
                                .await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(SoundcoreResponseMessage::Error(e.to_string()))
                                .await;
                        }
                    }
                }
                SoundcoreRequestMessage::OpenDevice(desc) => match state.new_device(&desc).await {
                    Ok(_) => {
                        let _ = tx.send(SoundcoreResponseMessage::CreationSuccess).await;
                        let mut device_state_receiver =
                            state.get_device().await.unwrap().subscribe_state();
                        let process_transmitter = tx.clone();
                        tokio::task::spawn(async move {
                            while let Ok(state) = device_state_receiver.recv().await {
                                let _ = process_transmitter
                                    .send(SoundcoreResponseMessage::NewState(state))
                                    .await;
                            }
                        });
                    }
                    Err(_) => {
                        let _ = tx
                            .send(SoundcoreResponseMessage::Error(
                                "Failed to create device".to_string(),
                            ))
                            .await;
                    }
                },
                SoundcoreRequestMessage::CloseDevice => {
                    state.device = None;
                    let _ = tx.send(SoundcoreResponseMessage::CreationSuccess).await;
                }
                SoundcoreRequestMessage::RequestNewState => match state.get_device().await {
                    Some(device) => {
                        let _ = device.refresh_state().await;
                    }
                    None => {
                        let _ = tx
                            .send(SoundcoreResponseMessage::Error(
                                "No device connected".to_string(),
                            ))
                            .await;
                    }
                },
                SoundcoreRequestMessage::SetSoundMode(packet) => {
                    todo!()
                }
                SoundcoreRequestMessage::SetEQ(eq) => match state.get_device().await {
                    Some(device) => {
                        let _ = device.set_eq(eq).await;
                    }
                    None => {
                        let _ = tx
                            .send(SoundcoreResponseMessage::Error(
                                "No device connected".to_string(),
                            ))
                            .await;
                    }
                },
            }
        }
    });
}
