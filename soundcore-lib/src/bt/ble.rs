use crate::error::SoundcoreResult;
use async_trait::async_trait;
use std::collections::HashSet;
use tokio::sync::mpsc;

#[async_trait]
pub trait BLEConnection {
    async fn name() -> SoundcoreResult<String>;
    async fn mac() -> SoundcoreResult<String>;
    async fn write(&self, data: &[u8], write_type: InternalWriteType) -> SoundcoreResult<()>;
    async fn receive_channel(&self) -> SoundcoreResult<mpsc::Receiver<Vec<u8>>>;
}

pub enum InternalWriteType {
    WithResponse,
    WithoutResponse,
}

pub trait BLEConnectionDescriptor {
    fn name(&self) -> &str;
    fn mac(&self) -> &str;
}

#[async_trait]
pub trait BLEConnectionRegistry {
    type BLEConnType: BLEConnection + Send + Sync;
    type BLEDescType: BLEConnectionDescriptor + Send + Sync;

    async fn descriptors(&self) -> SoundcoreResult<HashSet<Self::BLEDescType>>;
    async fn connection(&self, mac_addr: &str) -> SoundcoreResult<Option<Self::BLEConnType>>;
}

pub struct BLEConnectionUuidSet {
    pub service_uuid: uuid::Uuid,
    pub read_uuid: uuid::Uuid,
    pub write_uuid: uuid::Uuid,
}
