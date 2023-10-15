mod a3951;

pub use a3951::*;
use serde::{Deserialize, Serialize};

use crate::models::{DeviceFirmware, SerialNumber};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Default)]
pub struct DeviceInfoResponse {
    pub sn: Option<SerialNumber>,
    pub fw: Option<DeviceFirmware>,
}
