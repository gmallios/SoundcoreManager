use crate::ble::{BLEDeviceDescriptor, DeviceDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsBLEDescriptor {
    name: String,
    mac_addr: String,
}

impl WindowsBLEDescriptor {
    pub fn new(name: impl Into<String>, mac_addr: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mac_addr: mac_addr.into(),
        }
    }
}

impl DeviceDescriptor for WindowsBLEDescriptor {
    fn mac_addr(&self) -> &str {
        &self.mac_addr
    }

    fn name(&self) -> &str {
        &self.name
    }
}
