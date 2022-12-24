use crate::types::{Scanner, BluetoothDevice, BluetoothAdrr};
use async_trait::async_trait;
use iobluetooth::scan;

pub struct BthScanner {}

#[async_trait]
impl Scanner for BthScanner {
    fn new() -> BthScanner {
        BthScanner {}
    }
    
    async fn scan(&mut self) -> Vec<BluetoothDevice> {
        let devices = scan().await;
        let mut res = Vec::new();
        for dev in devices {
            res.push(BluetoothDevice {
                name: dev.name,
                address: BluetoothAdrr::from(dev.addr),
                connected: dev.is_connected,
                remembered: false,
            });
        }
        res
    }
}