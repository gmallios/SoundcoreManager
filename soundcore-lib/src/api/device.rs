use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

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
    fn new(connection: ConnectionType) -> SoundcoreResult<Self>
    where
        Self: Sized;
    fn get_supported_device(&self) -> SupportedModelIDs;
    fn get_device_name(&self) -> String;
    fn subscribe_state(&self) -> tokio::sync::broadcast::Receiver<SoundcoreDeviceState>;
}

#[enum_dispatch(SoundcoreDevice)]
pub enum SoundcoreDevices<ConnectionType> {
    A3951(A3951<ConnectionType>),
}
