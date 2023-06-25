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
        println!("Initial state: {:?}", initial_state);
        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let cureent_state_lock_clone = current_state_lock.clone();
        let (state_tx, _) = broadcast::channel::<SoundcoreDeviceState>(1);
        let state_tx_clone = state_tx.to_owned();
        let handle = tokio::spawn(async move {
            while let Some(bytes) = receive_channel.recv().await {
                println!("Received bytes: {:?}", bytes);
                match ResponsePackets::from_bytes(devices::SupportedModelIDs::A3951, &bytes) {
                    Some(packet) => {
                        /* TODO: Map packet to transformer */
                        if let Some(transformer) =
                            devices::a3951::transformers::packet_to_transformer(packet)
                        {
                            let mut state = cureent_state_lock_clone.write().await;
                            let new_state = transformer.transform(&state);
                            if new_state != *state {
                                *state = new_state;
                                if let Err(err) = state_tx_clone.send(new_state) {
                                    warn!("Failed to send state update: {:?}", err);
                                }
                            }
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
        todo!()
    }

    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        todo!()
    }
}
