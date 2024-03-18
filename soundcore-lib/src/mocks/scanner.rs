use std::{str::FromStr, time::Duration};

use async_trait::async_trait;

use crate::{
    ble::{BLEDeviceDescriptor, BLEDeviceScanner},
    error::SoundcoreLibResult,
};
use crate::btaddr::BluetoothAdrr;

pub struct MockBLEScanner;

#[async_trait]
impl BLEDeviceScanner for MockBLEScanner {
    async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        let descriptor = BLEDeviceDescriptor {
            addr: BluetoothAdrr::from_str("00:11:22:33:44:55").unwrap(),
            name: "Mock Soundcore Device".to_string(),
        };
        Ok(vec![descriptor])
    }
}
