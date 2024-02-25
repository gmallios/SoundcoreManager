use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::btaddr::BluetoothAdrr;
use crate::{
    ble::{BLEConnection, BLEDeviceDescriptor, WriteType},
    error::SoundcoreLibResult,
};

pub struct MockBLEConnection {
    read_channel_loop: Mutex<Option<tokio::sync::mpsc::Receiver<Vec<u8>>>>,
}

impl MockBLEConnection {
    pub fn new() -> Self {
        MockBLEConnection {
            read_channel_loop: Mutex::new(None),
        }
    }

    pub fn new_with_empty_channel() -> Self {
        MockBLEConnection {
            read_channel_loop: Mutex::new(Some(tokio::sync::mpsc::channel(1).1)),
        }
    }

    pub async fn set_read_channel(&self, channel: tokio::sync::mpsc::Receiver<Vec<u8>>) {
        let mut lock = self.read_channel_loop.lock().await;
        *lock = Some(channel);
    }
}

#[async_trait]
impl BLEConnection for MockBLEConnection {
    fn descriptor(&self) -> BLEDeviceDescriptor {
        BLEDeviceDescriptor {
            addr: BluetoothAdrr::from_str("00:11:22:33:44:55").unwrap(),
            name: "Mock Soundcore Device".to_string(),
        }
    }

    async fn byte_channel(&self) -> SoundcoreLibResult<tokio::sync::mpsc::Receiver<Vec<u8>>> {
        Ok(self.read_channel_loop.lock().await.take().unwrap())
    }

    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()> {
        // TODO: Implement it, no-op for now
        Ok(())
    }
}
