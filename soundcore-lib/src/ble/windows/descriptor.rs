use serde::{Deserialize, Serialize};

use crate::ble::DeviceDescriptor;
use crate::btaddr::BluetoothAdrr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsBLEDescriptor {
    name: String,
    addr: BluetoothAdrr,
}

impl WindowsBLEDescriptor {
    pub fn new(name: impl Into<String>, mac_addr: impl Into<BluetoothAdrr>) -> Self {
        Self {
            name: name.into(),
            addr: mac_addr.into(),
        }
    }
}

impl DeviceDescriptor for WindowsBLEDescriptor {
    fn mac_addr(&self) -> BluetoothAdrr {
        self.addr.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
