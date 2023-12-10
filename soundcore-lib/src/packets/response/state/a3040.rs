use serde::{Serialize, Deserialize};

use crate::models::{SingleBattery, FirmwareVer, SerialNumber};


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3040StateResponse {
    pub battery: SingleBattery,
    pub fw: FirmwareVer,
    pub sn: SerialNumber,
    
}