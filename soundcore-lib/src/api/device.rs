use std::sync::Arc;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::sync::{broadcast, mpsc::Receiver};

use crate::{
    api::SoundcoreDeviceState,
    bt::ble::BLEConnection,
    devices::{a3951::device::A3951, SupportedModelIDs},
    error::SoundcoreResult,
};

#[enum_dispatch]
#[async_trait]
pub trait SoundcoreDevice<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    /* TODO: Add Get/Set ANC,EQ,LDAC */
    async fn new(connection: Arc<ConnectionType>) -> SoundcoreResult<Self>
    where
        Self: Sized;
    async fn get_initial_state(
        connection: &Arc<ConnectionType>,
        receiver: &mut Receiver<Vec<u8>>,
    ) -> SoundcoreResult<SoundcoreDeviceState>;
    async fn name(&self) -> String;
    fn model_id(&self) -> SupportedModelIDs;
    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState>;
}

#[enum_dispatch(SoundcoreDevice)]
pub enum SoundcoreDevices<ConnectionType> {
    A3951(A3951<ConnectionType>),
}
