use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
#[typeshare]
pub struct StereoEQ {
    pub left: MonoEQ,
    pub right: MonoEQ,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
#[typeshare]
pub struct MonoEQ {
    /**
     * The values that we store are what is
     * received/sent and clamped within the range of 0..=240
     */
    pub values: Vec<u8>,
}

impl MonoEQ {
    pub const MIN_SIGNED: i8 = -120;
    pub const MAX_SIGNED: i8 = 120;
    pub const MIN_BYTE: u8 = 0;
    pub const MAX_BYTE: u8 = 240;
    pub const MAX_FLOAT: f32 = 18.0;
    pub const MIN_FLOAT: f32 = 6.0;

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            values: bytes.iter().map(|&v| Self::from_byte(&v)).collect(),
        }
    }


    pub fn to_bytes(&self, bands: usize) -> Vec<u8> {
        self.values.iter().take(bands).map(|&v| Self::to_byte(&v)).collect()
    }

    pub fn from_signed_bytes(bytes: Vec<i8>) -> Self {
        Self {
            values: bytes.iter().map(|&v| Self::from_signed(&v)).collect(),
        }
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self {
            values: bytes.iter().map(Self::from_byte).collect(),
        }
    }

    pub fn from_floats(floats: Vec<f32>) -> Self {
        Self {
            values: floats.iter().map(Self::from_float).collect(),
        }
    }

    pub fn to_floats(&self) -> Vec<f32> {
        self.values.iter().map(|&v| Self::to_float(v)).collect()
    }

    fn from_byte(value: &u8) -> u8 {
        value.clamp(&Self::MIN_BYTE, &Self::MAX_BYTE).to_owned()
    }

    fn to_byte(value: &u8) -> u8 {
        value.clamp(&Self::MIN_BYTE, &Self::MAX_BYTE).to_owned()
    }

    fn to_float(value: u8) -> f32 {
        ((value as f32) / 10.0).clamp(Self::MIN_FLOAT, Self::MAX_FLOAT)
    }

    fn from_float(value: &f32) -> u8 {
        (value * 10.0).clamp(Self::MIN_FLOAT, Self::MAX_FLOAT) as u8
    }


    /* Input should be clamped to -120..=120 then converted to the range 0..=240 */
    fn from_signed(value: &i8) -> u8 {
        value.clamp(&Self::MIN_SIGNED, &Self::MAX_SIGNED).wrapping_add(Self::MIN_SIGNED.abs()) as u8
    }

    fn to_signed(value: u8) -> i8 {
        (value as i8).wrapping_sub(Self::MIN_SIGNED.abs())
    }
}


#[cfg(test)]
mod eq_model {
    #[test]
    fn from_signed_check() {
        let signed_values = vec![-120, -100, -80, -60, -40, -20, 0, 20, 40, 60, 80, 100, 120];
        let expected_values = vec![0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240];
        let actual_values: Vec<u8> = signed_values.iter().map(super::MonoEQ::from_signed).collect();
        assert_eq!(expected_values, actual_values);
    }
}