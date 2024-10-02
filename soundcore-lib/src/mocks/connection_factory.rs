use crate::{
    ble::{BLEConnectionFactory, BLEConnectionUuidSet, BLEDeviceDescriptor},
    error::SoundcoreLibResult,
};

use super::MockBLEConnection;

pub struct MockBLEConnectionFactory;

impl BLEConnectionFactory for MockBLEConnectionFactory {
    type Connection = MockBLEConnection;

    async fn connect(
        &self,
        _descriptor: BLEDeviceDescriptor,
        _uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Self::Connection> {
        unimplemented!()
    }
}
