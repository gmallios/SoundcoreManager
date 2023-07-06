use std::sync::Arc;

use async_trait::async_trait;
use log::{trace, warn};
use tokio::{
    sync::{broadcast, mpsc::Receiver, RwLock},
    task::JoinHandle,
};

use crate::api::{BatteryLevel, ChargingStatus, EQValues, SoundMode};
use crate::{
    api::{
        RequestPacket, ResponsePackets, ResponseStateUpdatePackets, SoundcoreDevice,
        SoundcoreDeviceState,
    },
    bt::ble::{BLEConnection, InternalWriteType},
    devices::{self, a3951::packets::state_update::StateUpdateRequestPacket},
    error::{SoundcoreError, SoundcoreResult},
};

use super::packets::sound_mode::SoundModeRequestPacket;

pub struct A3951<ConnectionType> {
    connection: Arc<ConnectionType>,
    state: Arc<RwLock<SoundcoreDeviceState>>,
    receiver_handle: JoinHandle<()>,
    state_sender: broadcast::Sender<SoundcoreDeviceState>,
}

#[async_trait]
impl<ConnectionType> SoundcoreDevice<ConnectionType> for A3951<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    async fn new(connection: Arc<ConnectionType>) -> SoundcoreResult<Self>
    where
        Self: Sized,
    {
        let mut receive_channel = connection.receive_channel().await?;
        let initial_state = Self::get_initial_state(&connection, &mut receive_channel).await?;
        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let current_state_lock_clone = current_state_lock.clone();
        let (state_tx, _) = broadcast::channel::<SoundcoreDeviceState>(1);
        let state_tx_clone = state_tx.to_owned();

        let handle = tokio::spawn(async move {
            while let Some(bytes) = receive_channel.recv().await {
                match ResponsePackets::from_bytes(devices::SupportedModelIDs::A3951, &bytes) {
                    Some(packet) => {
                        if let Some(transformer) =
                            devices::a3951::transformers::packet_to_transformer(packet)
                        {
                            let mut state = current_state_lock_clone.write().await;
                            let new_state = transformer.transform(&state);
                            // TODO: Remove this
                            // if new_state != *state {
                            *state = new_state;
                            println!("Got new state: {:?}", new_state);
                            if let Err(err) = state_tx_clone.send(new_state) {
                                warn!("Failed to send state update: {:?}", err);
                            }
                            // }
                        }
                    }
                    None => {}
                }
            }
        });
        Ok(Self {
            connection,
            state: current_state_lock,
            receiver_handle: handle,
            state_sender: state_tx,
        })
    }

    async fn get_initial_state(
        connection: &Arc<ConnectionType>,
        receiver: &mut Receiver<Vec<u8>>,
    ) -> SoundcoreResult<SoundcoreDeviceState> {
        connection
            .write(
                &StateUpdateRequestPacket::new().bytes(),
                InternalWriteType::WithoutResponse,
            )
            .await?;
        let state_fut = async {
            while let Some(bytes) = receiver.recv().await {
                match ResponsePackets::from_bytes(devices::SupportedModelIDs::A3951, &bytes) {
                    Some(packet) => match packet {
                        ResponsePackets::StateUpdate(ResponseStateUpdatePackets::A3951(packet)) => {
                            return Some(SoundcoreDeviceState::from(packet))
                        }
                        _ => {
                            warn!("Received unexpected packet: {:?}", packet);
                        }
                    },
                    None => {
                        warn!("Received invalid packet {:?}", bytes);
                    }
                };
            }
            None
        };
        match tokio::time::timeout(std::time::Duration::from_secs(5), state_fut).await {
            Ok(state) => match state {
                Some(state) => return Ok(state),
                None => warn!("No state received from device"),
            },
            Err(_) => (),
        }
        Err(SoundcoreError::NoResponse)
    }

    fn model_id(&self) -> devices::SupportedModelIDs {
        todo!()
    }

    async fn name(&self) -> String {
        "Soundcore Liberty Air 2 Pro".to_owned()
    }

    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        self.state_sender.subscribe()
    }

    async fn refresh_state(&self) -> SoundcoreResult<()> {
        self.connection
            .write(
                &StateUpdateRequestPacket::new().bytes(),
                InternalWriteType::WithoutResponse,
            )
            .await
    }

    async fn set_sound_mode(&self, mode: SoundMode) -> SoundcoreResult<()> {
        println!(
            "Writing bytes: {:?}",
            SoundModeRequestPacket::new(mode).bytes()
        );
        self.connection
            .write(
                &SoundModeRequestPacket::new(mode).bytes(),
                InternalWriteType::WithResponse,
            )
            .await
    }

    async fn set_eq(&self, _eq: EQValues) -> SoundcoreResult<()> {
        todo!()
    }

    async fn battery_level(&self) -> BatteryLevel {
        self.state.read().await.battery_level
    }

    async fn charging_status(&self) -> ChargingStatus {
        self.state.read().await.charging_status
    }

    async fn sound_mode(&self) -> SoundMode {
        self.state.read().await.sound_mode
    }

    async fn eq(&self) -> EQValues {
        self.state.read().await.eq
    }
}

impl<ConnectionType> Drop for A3951<ConnectionType> {
    fn drop(&mut self) {
        self.receiver_handle.abort();
    }
}
