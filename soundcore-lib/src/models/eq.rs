use std::usize;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
pub struct StereoEQ {
    pub left: MonoEQ,
    pub right: MonoEQ,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
pub struct MonoEQ {
    pub values: [i8; 8],
}

impl MonoEQ {
    pub const MIN: i8 = -120;
    pub const MAX: i8 = 120;
    pub const MIN_BYTE: u8 = 0;
    pub const MAX_BYTE: u8 = 240;
    pub const MAX_FLOAT: f32 = 18.0;
    pub const MIN_FLOAT: f32 = 6.0;

    pub fn new(values: [i8; 8]) -> Self {
        Self {
            values: values.map(|v| v.clamp(Self::MIN, Self::MAX)),
        }
    }

    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        Self {
            values: bytes.map(Self::from_byte),
        }
    }

    pub fn from_floats(floats: &[f32; 8]) -> Self {
        Self {
            values: floats.map(Self::from_float),
        }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        self.values.map(Self::to_byte)
    }

    pub fn to_floats(&self) -> [f32; 8] {
        self.values.map(Self::to_float)
    }

    /* Transforms the byte to the applicable u8 range */
    fn to_byte(value: i8) -> u8 {
        value
            .clamp(Self::MIN, Self::MAX)
            .wrapping_add(Self::MIN.abs()) as u8
    }

    /* Transforms the byte to the applicable i8 range */
    fn from_byte(value: u8) -> i8 {
        value
            .clamp(Self::MIN_BYTE, Self::MAX_BYTE)
            .wrapping_sub(Self::MIN.unsigned_abs()) as i8
    }

    fn to_float(_value: i8) -> f32 {
        todo!()
    }

    fn from_float(_value: f32) -> i8 {
        todo!()
    }
}
