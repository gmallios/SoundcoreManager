use std::sync::Arc;
use std::time::Duration;

use log::{debug, error, trace};
use tokio::sync::{mpsc, Mutex, watch};
use tokio::task::JoinHandle;
use tokio::time::timeout;

use crate::api::SoundcoreDeviceState;
use crate::ble::{BLEConnection, WriteType};
use crate::error::{SoundcoreLibError, SoundcoreLibResult};
use crate::models::{EQConfiguration, SoundMode};
use crate::packets::{
    DeviceStateResponse, RequestPacketBuilder, RequestPacketKind, ResponsePacket,
    StateTransformationPacket,
};
use crate::parsers::TaggedData;
use crate::types::SupportedModels;

pub struct SoundcoreBLEDevice<Connection>
    where
        Connection: BLEConnection,
{
    connection: Arc<Connection>,
    state_channel: Arc<Mutex<watch::Sender<SoundcoreDeviceState>>>,
    state_channel_handle: JoinHandle<()>,
    model: Option<SupportedModels>,
}

impl<Connection> SoundcoreBLEDevice<Connection>
    where
        Connection: BLEConnection + Send + Sync,
{
    pub async fn new(connection: Arc<Connection>) -> SoundcoreLibResult<Self> {
        let mut byte_channel = connection.byte_channel().await?;
        let initial_state = Self::init_state(&connection, &mut byte_channel).await?;
        debug!(
            "initial State: {:?}, device: {:?}",
            initial_state,
            connection.descriptor()
        );
        let state_sender = Arc::new(Mutex::new(watch::channel(initial_state.data).0));
        let packet_handler = Self::spawn_packet_handler(state_sender.to_owned(), byte_channel);
        Ok(Self {
            connection,
            state_channel: state_sender,
            state_channel_handle: packet_handler,
            model: Some(initial_state.tag),
        })
    }

    async fn init_state(
        connection: &Connection,
        byte_channel: &mut mpsc::Receiver<Vec<u8>>,
    ) -> SoundcoreLibResult<TaggedData<SoundcoreDeviceState>> {
        let initial_state = Self::fetch_initial_state(connection, byte_channel).await?;
        // Todo: If something is missing (e.g. Firmware Version or Serial Number) fetch it
        Ok(initial_state)
    }

    async fn fetch_initial_state(
        connection: &Connection,
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
                match timeout(Duration::from_millis(1000), state_receive_fut).await {
                    Ok(Some(packet)) => {
                        return Ok(packet);
                    }
                    _ => {}
                };
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
            retry_count += 1;
        }
        Err(SoundcoreLibError::MissingInitialState(
            connection.descriptor().addr.to_string(),
        ))
    }

    fn spawn_packet_handler(
        state_sender: Arc<Mutex<watch::Sender<SoundcoreDeviceState>>>,
        mut byte_channel: mpsc::Receiver<Vec<u8>>,
    ) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            while let Some(bytes) = byte_channel.recv().await {
                trace!("Received bytes: {:?}", bytes);
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

    async fn set_sound_mode(&self, sound_mode: SoundMode) -> SoundcoreLibResult<()> {
        todo!()
    }

    async fn set_eq(&self, eq: EQConfiguration) -> SoundcoreLibResult<()> {
        todo!()
    }
}
