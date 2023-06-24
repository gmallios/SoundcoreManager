use std::sync::Arc;

use async_trait::async_trait;
use log::warn;
use tokio::{
    sync::{broadcast, mpsc::Receiver, RwLock},
    task::JoinHandle,
};

use crate::{
    api::{
        RequestPacket, ResponsePackets, ResponseStateUpdatePackets, SoundcoreDevice,
        SoundcoreDeviceState,
    },
    bt::ble::{BLEConnection, InternalWriteType},
    devices::{self, a3951::packets::state_update::StateUpdateRequestPacket},
    error::{SoundcoreError, SoundcoreResult},
};

pub struct A3951<ConnectionType> {
    connection: ConnectionType,
    state: Arc<RwLock<SoundcoreDeviceState>>,
    receiver_handle: JoinHandle<()>,
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
        println!("Initial state: {:?}", initial_state);
        let _current_state_lock = Arc::new(RwLock::new(initial_state));
        let (state_tx, _) = broadcast::channel::<SoundcoreDeviceState>(1);
        let _state_tx_clone = state_tx;
        let _handle = tokio::spawn(async move {
            while let Some(bytes) = receive_channel.recv().await {
                println!("Received bytes: {:?}", bytes);
                match ResponsePackets::from_bytes(devices::SupportedModelIDs::A3951, &bytes) {
                    Some(_packet) => {
                        /* TODO: Map packet to transformer */
                        todo!()
                    }
                    None => {}
                }
            }
        });
        todo!()
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
        todo!()
    }

    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        todo!()
    }
}
