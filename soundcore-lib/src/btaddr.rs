use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::error::{SoundcoreLibError, SoundcoreLibResult};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typeshare]
pub struct BluetoothAdrr {
    pub address: [u8; 6],
}

impl Debug for BluetoothAdrr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl BluetoothAdrr {
    pub const SOUNDCORE_MAC_PREFIXES: [[u8; 3]; 2] = [[0xAC, 0x12, 0x2F], [0xE8, 0xEE, 0xCC]];

    pub fn from_str(address: &str) -> SoundcoreLibResult<Self> {
        match address.contains(':') {
            true => Self::from_colon_str(address),
            false => Self::from_dash_str(address),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> SoundcoreLibResult<Self> {
        if bytes.len() != 6 {
            return Err(SoundcoreLibError::InvalidMACAddress {
                addr: format!("{:?}", bytes),
            });
        }
        Ok(Self {
            address: bytes.try_into().unwrap(),
        })
    }

    fn from_colon_str(address: &str) -> SoundcoreLibResult<Self> {
        let addr = address
            .split(':')
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| SoundcoreLibError::InvalidMACAddress {
                addr: address.into(),
            })?;
        Ok(Self {
            address: addr
                .try_into()
                .map_err(|_| SoundcoreLibError::InvalidMACAddress {
                    addr: address.into(),
                })?,
        })
    }

    fn from_dash_str(address: &str) -> SoundcoreLibResult<Self> {
        let addr = address
            .split('-')
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| SoundcoreLibError::InvalidMACAddress {
                addr: address.into(),
            })?;
        Ok(Self {
            address: addr
                .try_into()
                .map_err(|_| SoundcoreLibError::InvalidMACAddress {
                    addr: address.into(),
                })?,
        })
    }

    pub fn is_soundcore_mac(&self) -> bool {
        Self::SOUNDCORE_MAC_PREFIXES
            .iter()
            .any(|prefix| self.address.starts_with(prefix))
    }
}

impl From<BluetoothAdrr> for String {
    fn from(val: BluetoothAdrr) -> Self {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            val.address[0],
            val.address[1],
            val.address[2],
            val.address[3],
            val.address[4],
            val.address[5]
        )
    }
}

/// Windows Bluetooth Address
impl From<u64> for BluetoothAdrr {
    fn from(value: u64) -> Self {
        let bytes = value.to_be_bytes();
        BluetoothAdrr {
            address: [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]],
        }
    }
}

impl From<BluetoothAdrr> for u64 {
    fn from(value: BluetoothAdrr) -> Self {
        let mut bytes = [0u8; 8];
        bytes[2..].copy_from_slice(&value.address);
        u64::from_be_bytes(bytes)
    }
}

impl Display for BluetoothAdrr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5],
        )
    }
}

#[cfg(test)]
mod mac_tests {
    use super::*;

    #[test]
    fn from_string_with_colon_separator() {
        let address_str = String::from("11:22:33:44:55:66");
        let address = BluetoothAdrr::from_str(&address_str).unwrap();

        assert_eq!(address.address, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    }

    #[test]
    fn from_string_with_dash_separator() {
        let address_str = String::from("11-22-33-44-55-66");
        let address = BluetoothAdrr::from_str(&address_str).unwrap();

        assert_eq!(address.address, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    }

    #[test]
    fn into_string() {
        let address = BluetoothAdrr {
            address: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        };

        let address_str: String = address.into();

        assert_eq!(address_str, "11:22:33:44:55:66");
    }

    #[test]
    fn display_formatting() {
        let address = BluetoothAdrr {
            address: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        };

        let formatted_address = format!("{}", address);

        assert_eq!(formatted_address, "11:22:33:44:55:66");
    }

    #[test]
    fn check_soundcore_mac() {
        let address = BluetoothAdrr {
            address: [0xAC, 0x12, 0x2F, 0x44, 0x55, 0x66],
        };

        assert!(address.is_soundcore_mac());

        let non_soundcore = BluetoothAdrr {
            address: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        };

        assert!(!non_soundcore.is_soundcore_mac());
    }

    #[test]
    fn test_from_windows_u64() {
        let address_value: u64 = 0xB123456789AB;
        let address = BluetoothAdrr::from(address_value);
        assert_eq!(address.address, [0xB1, 0x23, 0x45, 0x67, 0x89, 0xAB])
    }

    #[test]
    fn test_from_windows_u64_to_string() {
        let address_value: u64 = 0xB123456789AB;
        let address = BluetoothAdrr::from(address_value);
        assert_eq!(address.to_string(), "B1:23:45:67:89:AB")
    }

    #[test]
    fn test_into_windows_u64() {
        let address = BluetoothAdrr {
            address: [0x33, 0x44, 0x55, 0x66, 0x77, 0x00],
        };
        let address_value: u64 = address.into();
        assert_eq!(address_value, 0x334455667700);
    }
}
