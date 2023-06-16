use crate::{
    api::{SoundcoreDevice, SoundcoreDeviceState},
    bt::ble::BLEConnection,
};

pub struct A3951<ConnectionType> {
    connection: ConnectionType,
}

impl<ConnectionType> SoundcoreDevice<ConnectionType> for A3951<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    fn get_supported_device(&self) -> crate::devices::SupportedModelIDs {
        todo!()
    }

    fn get_device_name(&self) -> String {
        todo!()
    }

    fn subscribe_state(&self) -> tokio::sync::broadcast::Receiver<SoundcoreDeviceState> {
        todo!()
    }

    fn new(_connection: ConnectionType) -> crate::error::SoundcoreResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
