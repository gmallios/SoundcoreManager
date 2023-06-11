use std::fmt::Display;

use crate::error::{SoundcoreError, SoundcoreResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothAdrr {
    pub address: [u8; 6],
}

impl BluetoothAdrr {
    pub fn from_str(address: &str) -> SoundcoreResult<Self> {
        match address.contains(':') {
            true => Self::from_colon_str(address),
            false => Self::from_dash_str(address),
        }
    }

    fn from_colon_str(address: &str) -> SoundcoreResult<Self> {
        let addr = address
            .split(':')
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| SoundcoreError::InvalidMACAddress {
                addr: address.into(),
            })?;
        Ok(Self {
            address: addr
                .try_into()
                .map_err(|_| SoundcoreError::InvalidMACAddress {
                    addr: address.into(),
                })?,
        })
    }

    fn from_dash_str(address: &str) -> SoundcoreResult<Self> {
        let addr = address
            .split('-')
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| SoundcoreError::InvalidMACAddress {
                addr: address.into(),
            })?;
        Ok(Self {
            address: addr
                .try_into()
                .map_err(|_| SoundcoreError::InvalidMACAddress {
                    addr: address.into(),
                })?,
        })
    }
}

impl Into<String> for BluetoothAdrr {
    fn into(self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5]
        )
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
}
