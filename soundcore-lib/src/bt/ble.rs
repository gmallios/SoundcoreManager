use crate::error::SoundcoreResult;
use async_trait::async_trait;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::mpsc;

#[async_trait]
pub trait BLEConnection {
    async fn name(&self) -> SoundcoreResult<String>;
    async fn mac(&self) -> SoundcoreResult<String>;
    async fn write(&self, data: &[u8], write_type: InternalWriteType) -> SoundcoreResult<()>;
    async fn receive_channel(&self) -> SoundcoreResult<mpsc::Receiver<Vec<u8>>>;
}

pub enum InternalWriteType {
    WithResponse,
    WithoutResponse,
}

pub trait BLEDeviceDescriptor {
    fn name(&self) -> &str;
    fn mac(&self) -> &str;
}

#[async_trait]
pub trait BLEConnectionRegistry {
    type ConnType: BLEConnection + Send + Sync;
    type DescType: BLEDeviceDescriptor + Send + Sync;

    async fn descriptors(&self) -> SoundcoreResult<HashSet<Self::DescType>>;
    async fn connection(
        &self,
        mac_addr: &str,
        uuid_set: BLEConnectionUuidSet,
    ) -> SoundcoreResult<Option<Arc<Self::ConnType>>>;
}

pub struct BLEConnectionUuidSet {
    pub service_uuid: uuid::Uuid,
    pub read_uuid: uuid::Uuid,
    pub write_uuid: uuid::Uuid,
}
