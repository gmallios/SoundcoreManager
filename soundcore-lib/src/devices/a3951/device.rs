use std::sync::Arc;

use async_trait::async_trait;
use tokio::{
    sync::{broadcast, RwLock},
    task::JoinHandle,
};

use crate::{
    api::{ResponsePackets, SoundcoreDevice, SoundcoreDeviceState},
    bt::ble::BLEConnection,
    devices,
    error::SoundcoreResult,
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
        let initial_state = Self::get_initial_state().await?;
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

    async fn get_initial_state() -> SoundcoreResult<SoundcoreDeviceState> {
        todo!()
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
