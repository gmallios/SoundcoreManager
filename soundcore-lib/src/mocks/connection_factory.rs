use async_trait::async_trait;

use crate::{
    ble::{BLEConnectionFactory, BLEConnectionUuidSet, BLEDeviceDescriptor},
    error::SoundcoreLibResult,
};

use super::MockBLEConnection;

pub struct MockBLEConnectionFactory;

#[async_trait]
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
