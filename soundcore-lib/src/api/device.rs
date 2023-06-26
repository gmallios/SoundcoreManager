use crate::{
    api::SoundcoreDeviceState,
    bt::ble::BLEConnection,
    devices::{a3951::device::A3951, SupportedModelIDs},
    error::SoundcoreResult,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc::Receiver};

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
    ) -> SoundcoreResult<SoundcoreDeviceState>
    where
        Self: Sized;
    async fn name(&self) -> String;
    fn model_id(&self) -> SupportedModelIDs;
    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState>;
}

#[enum_dispatch(SoundcoreDevice)]
pub enum SoundcoreDevices<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    A3951(A3951<ConnectionType>),
}

impl<ConnectionType> SoundcoreDevices<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    pub fn downcast(&self) -> Box<&dyn SoundcoreDevice<ConnectionType>> {
        match self {
            SoundcoreDevices::A3951(device) => Box::new(device.to_owned()),
        }
    }

    pub fn to_device(&self) -> &impl SoundcoreDevice<ConnectionType> {
        match self {
            SoundcoreDevices::A3951(device) => device.to_owned(),
        }
    }

    pub fn model_id(&self) -> SupportedModelIDs {
        match self {
            SoundcoreDevices::A3951(device) => device.model_id(),
        }
    }

    pub fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        match self {
            SoundcoreDevices::A3951(device) => device.subscribe_state(),
        }
    }

    fn check(&self) {
        let device = self.to_device();
    }
}
