use std::str::FromStr;

use btleplug::{api::BDAddr, platform::PeripheralId};

use crate::btaddr::BluetoothAdrr;
use crate::{ble::WriteType, error::SoundcoreLibError};

impl TryFrom<BDAddr> for BluetoothAdrr {
    type Error = SoundcoreLibError;

    fn try_from(value: BDAddr) -> Result<Self, Self::Error> {
        BluetoothAdrr::from_bytes(&value.into_inner())
    }
}

impl From<BluetoothAdrr> for BDAddr {
    fn from(value: BluetoothAdrr) -> Self {
        value.address.into()
    }
}

impl TryFrom<PeripheralId> for BluetoothAdrr {
    type Error = SoundcoreLibError;

    fn try_from(value: PeripheralId) -> Result<Self, Self::Error> {
        BluetoothAdrr::from_str(&value.to_string())
    }
}

impl From<WriteType> for btleplug::api::WriteType {
    fn from(value: WriteType) -> Self {
        match value {
            WriteType::WithoutResponse => btleplug::api::WriteType::WithoutResponse,
            WriteType::WithResponse => btleplug::api::WriteType::WithResponse,
        }
    }
}
