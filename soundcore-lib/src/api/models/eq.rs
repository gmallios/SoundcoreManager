use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EQValues {
    pub values: [i8; 8],
}

impl EQValues {
    pub const MIN: i8 = -120;
    pub const MAX: i8 = 120;

    pub fn new(values: [i8; 8]) -> Self {
        Self {
            values: values.map(|v| v.clamp(Self::MIN, Self::MAX)),
        }
    }

    pub fn values(&self) -> [i8; 8] {
        self.values
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new(bytes.map(Self::from_packet_byte))
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.values.map(Self::to_packet_byte)
    }

    fn to_packet_byte(v: i8) -> u8 {
        v.clamp(Self::MIN, Self::MAX).wrapping_add(Self::MIN.abs()) as u8
    }

    fn from_packet_byte(byte: u8) -> i8 {
        byte.clamp(
            Self::to_packet_byte(Self::MIN),
            Self::to_packet_byte(Self::MAX),
        )
        .wrapping_sub(Self::MIN.unsigned_abs()) as i8
    }
}

#[cfg(test)]
mod tests {
    use super::EQValues;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [i8; 8] = [-120, -40, -20, 0, 20, 40, 60, 120];

    #[test]
    fn to_bytes_valid() {
        let eq_values = EQValues::new(TEST_ADJUSTMENTS);
        assert_eq!(TEST_BYTES, eq_values.bytes());
    }

    #[test]
    fn from_bytes_valid() {
        let eq_values = EQValues::from_bytes(TEST_BYTES);
        assert_eq!(TEST_ADJUSTMENTS, eq_values.values());
    }

    #[test]
    fn clamps_from_bytes() {
        let eq_values = EQValues::from_bytes([0, 255, 120, 120, 120, 120, 120, 120]);
        assert_eq!([0, 240, 120, 120, 120, 120, 120, 120], eq_values.bytes());
    }

    #[test]
    fn clamps_from_i8() {
        let eq_values = EQValues::new([-128, 127, 0, 0, 0, 0, 0, 0]);
        assert_eq!([-120, 120, 0, 0, 0, 0, 0, 0], eq_values.values());
    }
}
