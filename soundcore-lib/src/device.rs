use std::sync::Arc;
use std::time::Duration;

use log::{debug, error, trace};
use tokio::sync::{mpsc, watch, Mutex};

use manager_fut::ManagerFuture;

use crate::api::SoundcoreDeviceState;
use crate::ble::{BLEConnection, WriteType};
use crate::error::{SoundcoreLibError, SoundcoreLibResult};
use crate::models::{BassUp, EQConfiguration, EQProfile, SoundMode};
use crate::packets::{
    BassUpCommandBuilder, EqCommandBuilder, RequestPacketBuilder, RequestPacketKind,
    ResponsePacket, SoundModeCommandBuilder, StateTransformationPacket,
};
use crate::parsers::TaggedData;
use crate::types::KnownProductCodes;

pub struct SoundcoreBLEDevice<C, F>
where
    C: BLEConnection,
    F: ManagerFuture,
{
    connection: Arc<C>,
    state_channel: Arc<Mutex<watch::Sender<SoundcoreDeviceState>>>,
    state_channel_handle: F::JoinHandle,
    model: KnownProductCodes,
}

impl<C, F> SoundcoreBLEDevice<C, F>
where
    C: BLEConnection,
    F: ManagerFuture,
{
    pub async fn new(connection: Arc<C>) -> SoundcoreLibResult<Self> {
        let mut byte_channel = connection.byte_channel().await?;
        let initial_state = Self::init_state(&connection, &mut byte_channel).await?;
        debug!(
            "initial State: {:?}, device: {:?}",
            initial_state,
            connection.descriptor()
        );
        let state_sender = Arc::new(Mutex::new(watch::channel(initial_state.data.clone()).0));
        let packet_handler = Self::spawn_packet_handler(state_sender.to_owned(), byte_channel);

        let model = if let Some(sn) = initial_state.data.serial {
            sn.to_model().unwrap_or(initial_state.tag)
        } else {
            initial_state.tag
        };

        Ok(Self {
            connection,
            state_channel: state_sender,
            state_channel_handle: packet_handler,
            model,
        })
    }

    async fn init_state(
        connection: &C,
        byte_channel: &mut mpsc::Receiver<Vec<u8>>,
    ) -> SoundcoreLibResult<TaggedData<SoundcoreDeviceState>> {
        let initial_state = Self::fetch_initial_state(connection, byte_channel).await?;
        // Todo: If something is missing (e.g. Firmware Version or Serial Number) fetch it
        Ok(initial_state)
    }

    // TODO: Change the strategy to:
    // 1. Send the state request packet
    // 2. Check if the state is received within a certain time frame and retry if not
    // 3. If the state is received, check if we have a SN
    // 4. If not send a SN request packet
    // 5. Resolve the state and the model
    async fn fetch_initial_state(
        connection: &C,
        byte_channel: &mut mpsc::Receiver<Vec<u8>>,
    ) -> SoundcoreLibResult<TaggedData<SoundcoreDeviceState>> {
        let packet = RequestPacketBuilder::new(RequestPacketKind::State).build();
        let mut retry_count = 0;
        let retry_limit = 3;
        while retry_count < retry_limit {
            let state_send_fut = async {
                connection
                    .write(&packet, WriteType::WithoutResponse)
                    .await?;
                Ok::<(), SoundcoreLibError>(())
            };

            let state_receive_fut = async {
                while let Some(bytes) = byte_channel.recv().await {
                    match ResponsePacket::from_bytes_for_initial_state(&bytes) {
                        Ok(Some(packet)) => {
                            return Some(packet);
                        }
                        Err(e) => {
                            error!("Failed to parse state packet: {:?}", e);
                        }
                        _ => {}
                    }
                }
                None
            };

            if let Ok(_) = state_send_fut.await {
                match F::timeout(Duration::from_millis(1000), state_receive_fut).await {
                    Ok(Some(packet)) => {
                        return Ok(packet);
                    }
                    _ => {}
                };
            }

            F::sleep(Duration::from_millis(500)).await;
            retry_count += 1;
        }

        // TODO: Add test data to mocked device so initial state can be fetched
        if cfg!(test) || cfg!(feature = "mock") {
            return Ok(TaggedData {
                tag: KnownProductCodes::A3951,
                data: SoundcoreDeviceState::default(),
            });
        }

        Err(SoundcoreLibError::MissingInitialState(
            connection.descriptor().addr.to_string(),
        ))
    }

    fn spawn_packet_handler(
        state_sender: Arc<Mutex<watch::Sender<SoundcoreDeviceState>>>,
        mut byte_channel: mpsc::Receiver<Vec<u8>>,
    ) -> F::JoinHandle {
        F::spawn(async move {
            while let Some(bytes) = byte_channel.recv().await {
                trace!("Received bytes: {:?}", bytes);
                if bytes.is_empty() {
                    continue;
                }

                match ResponsePacket::from_bytes(&bytes) {
                    Ok(packet) => {
                        let state_sender = state_sender.lock().await;
                        let old_state = state_sender.borrow();
                        let new_state = packet.transform_state(&old_state);
                        if new_state != *old_state {
                            std::mem::drop(old_state);
                            state_sender.send_replace(new_state);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse packet: {:?}", e);
                    }
                    _ => {
                        error!("Unknown packet received, bytes: {:?}", bytes);
                    }
                }
            }
            trace!("Packet handler finished");
        })
    }

    pub async fn state_channel(&self) -> watch::Receiver<SoundcoreDeviceState> {
        self.state_channel.lock().await.subscribe()
    }

    pub async fn latest_state(&self) -> SoundcoreDeviceState {
        self.state_channel.lock().await.borrow().clone()
    }

    pub async fn set_sound_mode(&self, sound_mode: SoundMode) -> SoundcoreLibResult<()> {
        // TODO: perform some validation on the sound mode/features
        // TODO: Check if https://github.com/Oppzippy/OpenSCQ30/blob/dec0ad3f2659205ff6efdb8d12ec333ba9f3a0b4/lib/src/soundcore_device/device/device_command_dispatcher.rs#L28
        // is valid for all models or device-specific
        let command = SoundModeCommandBuilder::new(sound_mode, self.model).build();
        let latest_state = self.latest_state().await;

        if latest_state.sound_mode == sound_mode {
            return Ok(());
        }

        self.connection
            .write(&command, WriteType::WithoutResponse)
            .await?;

        let state_sender = self.state_channel.lock().await;
        let mut new_state = state_sender.borrow().clone();
        new_state.sound_mode = sound_mode;
        state_sender.send_replace(new_state);

        Ok(())
    }

    pub async fn set_eq(&self, eq: EQConfiguration) -> SoundcoreLibResult<()> {
        let latest_state = self.latest_state().await;
        let state_sender = self.state_channel.lock().await;
        let mut new_state = state_sender.borrow().clone();
        // If the device supports bass up, the transition from
        // SoundcoreSignature<->BassBooster should be mapped to
        // a BassUp command. Additionally, if the transition is
        // from BassBooster->SoundcoreSignature send the eq command
        // after the BassUp.
        if let Some(features) = latest_state.feature_set.equalizer_features {
            let latest_eq_profile = latest_state.eq_configuration.get_profile();
            let new_eq_profile = eq.get_profile();
            if features.has_bass_up {
                trace!(
                    "Device {:?} supports BassUp, building and sending the appropriate command...",
                    self.model
                );
                if latest_eq_profile == EQProfile::SoundcoreSignature
                    && new_eq_profile == EQProfile::BassBooster
                {
                    self.connection
                        .write(
                            &BassUpCommandBuilder::new(self.model, true).build(),
                            WriteType::WithoutResponse,
                        )
                        .await?;
                    new_state.bass_up = Some(BassUp(true));
                    let mut new_eq = new_state.eq_configuration.clone();
                    new_eq.set_profile(EQProfile::BassBooster);
                    new_state.eq_configuration = new_eq;
                    state_sender.send_replace(new_state);
                    return Ok(());
                } else if latest_eq_profile == EQProfile::BassBooster
                    && new_eq_profile == EQProfile::SoundcoreSignature
                {
                    self.connection
                        .write(
                            &BassUpCommandBuilder::new(self.model, false).build(),
                            WriteType::WithoutResponse,
                        )
                        .await?;
                    new_state.bass_up = new_state.bass_up.map(|_| BassUp(false));
                }
            }
        }

        let command = EqCommandBuilder::new(eq.clone(), self.model).build();

        self.connection
            .write(&command, WriteType::WithoutResponse)
            .await?;

        new_state.eq_configuration = eq;
        state_sender.send_replace(new_state);

        Ok(())
    }
}
