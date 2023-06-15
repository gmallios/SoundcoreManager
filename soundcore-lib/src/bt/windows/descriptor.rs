use crate::bt::ble::BLEDeviceDescriptor;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl BLEDeviceDescriptor for WindowsBLEDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac(&self) -> &str {
        &self.mac_addr
    }
}
