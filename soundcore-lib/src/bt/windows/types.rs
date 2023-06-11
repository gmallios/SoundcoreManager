use crate::mac::BluetoothAdrr;

impl From<u64> for BluetoothAdrr {
    fn from(address: u64) -> Self {
        /* used for BluetoothDevice.BluetoothAddress() from windows crate */
        let bytes = address.to_be_bytes();
        BluetoothAdrr {
            address: [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]],
        }
    }
}

impl From<BluetoothAdrr> for u64 {
    fn from(val: BluetoothAdrr) -> Self {
        let mut bytes = [0u8; 8];
        bytes[2..8].copy_from_slice(&val.address);
        u64::from_be_bytes(bytes)
    }
}

#[cfg(test)]
mod mac_tests {
    use super::*;

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
        println!("{:X}", address_value);

        assert_eq!(address_value, 0x334455667700);
    }
}
