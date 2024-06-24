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
    pub const DRC_ADJUSTMENT: f32 = 10.0;

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            values: bytes.iter().map(|&v| Self::from_byte(&v)).collect(),
        }
    }

    pub fn to_bytes(&self, bands: usize) -> Vec<u8> {
        self.values
            .iter()
            .take(bands)
            .map(|&v| Self::to_byte(&v))
            .collect()
    }

    pub fn to_10band_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self
            .values
            .iter()
            .take(8)
            .map(|&v| Self::to_byte(&v))
            .collect();
        bytes.push(120u8);
        bytes.push(0u8);
        bytes
    }

    pub fn to_drc_bytes(&self) -> Vec<u8> {
        Self::calculate_drc_adjustments(
            self.values
                .iter()
                .take(8usize)
                .map(|&v| (v as f32 / Self::DRC_ADJUSTMENT) - (Self::MAX_FLOAT - Self::MIN_FLOAT))
                .collect(),
        )
        .iter()
        .map(|&v| ((v + (Self::MAX_FLOAT - Self::MIN_FLOAT)) * Self::DRC_ADJUSTMENT).round() as u8) // Apply DRC adjustments to middle point
        .collect()
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
            values: floats.iter().map(Self::from_small_range_float).collect(),
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

    /* Used for range of -6..=6 */
    fn from_small_range_float(value: &f32) -> u8 {
        Self::from_signed(&((value * 20.0) as i8).clamp(Self::MIN_SIGNED, Self::MAX_SIGNED))
    }

    /* Input should be clamped to -120..=120 then converted to the range 0..=240 */
    fn from_signed(value: &i8) -> u8 {
        value
            .clamp(&Self::MIN_SIGNED, &Self::MAX_SIGNED)
            .wrapping_add(Self::MIN_SIGNED.abs()) as u8
    }

    fn to_signed(value: u8) -> i8 {
        (value as i8).wrapping_sub(Self::MIN_SIGNED.abs())
    }

    fn calculate_drc_adjustments(values: Vec<f32>) -> Vec<f32> {
        // Input floats should be in the range ((MIN_FLOAT - MIN_FLOAT)  - (MAX_FLOAT - MIN_FLOAT)) and length >= 8
        assert!(values.len() >= 8);
        assert!(values.iter().all(|&v| v >= (-6.0) && v <= 6.0));

        // f64s is required to match the original implementation
        let d = values[0] as f64;
        let d2 = values[1] as f64;
        let d3 = 0.85f64;
        let d4 = values[2] as f64;
        let d5 = (1.26f64 * d) - ((d2 * 0.71f64) * d3) + (d4 * 0.177f64);
        let d6 = values[3] as f64;
        let d7 = values[4] as f64;
        let d8 = (d5 - (d6 * 0.0494f64)) + (d7 * 0.0345f64);
        let d9 = values[5] as f64;
        let d10 = values[6] as f64;
        let d11 = (d8 - (d9 * 0.0197f64)) + (d10 * 0.0075f64);
        let d12 = values[7] as f64;
        let d13 = (-0.71f64) * d * d3;
        let d14 = 0.95f64;
        let d15 = d4 * 0.81f64 * d3;
        let d16 = d9 * 0.81f64 * d3;

        vec![
            (d11 - (0.00217f64 * d12)) as f32,
            (((((((d13 + ((d2 * 1.73f64) * d14)) - d15) + (d6 * 0.204f64)) - (d7 * 0.068f64))
                + (d9 * 0.045f64))
                - (d10 * 0.0235f64))
                + (d12 * 0.0075f64)) as f32,
            ((((((((d * 0.177f64) - ((d2 * 0.81f64) * d3)) + ((d4 * 1.73f64) * d14))
                - ((d6 * 0.81f64) * d3))
                + (d7 * 0.208f64))
                - (d9 * 0.07f64))
                + (d10 * 0.045f64))
                - (d12 * 0.0197f64)) as f32,
            (((((((((-0.0494f64) * d) + (d2 * 0.204f64)) - d15) + ((d6 * 1.73f64) * d14))
                - ((d7 * 0.82f64) * d3))
                + (d9 * 0.208f64))
                - (d10 * 0.068f64))
                + (d12 * 0.0345f64)) as f32,
            ((((((((d * 0.0345f64) - (d2 * 0.068f64)) + (d4 * 0.208f64))
                - ((0.82f64 * d6) * d3))
                + ((d7 * 1.73f64) * d14))
                - d16)
                + (d10 * 0.204f64))
                - (d12 * 0.0494f64)) as f32,
            (((((((((-0.0197f64) * d) + (d2 * 0.045f64)) - (0.07f64 * d4)) + (0.208f64 * d6))
                - ((d7 * 0.81f64) * d3))
                + ((1.73f64 * d9) * d14))
                - ((d10 * 0.81f64) * d3))
                + (d12 * 0.177f64)) as f32,
            ((((((((d * 0.0075f64) - (d2 * 0.0235f64)) + (d4 * 0.045f64)) - (0.068f64 * d6))
                + (d7 * 0.204f64))
                - d16)
                + ((1.83f64 * d10) * d14))
                - ((d12 * 0.71f64) * d3)) as f32,
            ((((((((d * (-0.00217f64)) + (d2 * 0.0075f64)) - (d4 * 0.0197f64))
                + (d6 * 0.0345f64))
                - (d7 * 0.0494f64))
                + (d9 * 0.177f64))
                - ((d10 * 0.71f64) * d3))
                + (d12 * 1.5f64)) as f32,
            0f32,
            -120f32,
        ]
        .iter()
        .map(|v| v / 10.0)
        .collect()
    }
}

impl From<MonoEQ> for StereoEQ {
    fn from(eq: MonoEQ) -> Self {
        Self {
            left: eq.clone(),
            right: eq,
        }
    }
}

impl From<StereoEQ> for MonoEQ {
    fn from(eq: StereoEQ) -> Self {
        eq.left.clone()
    }
}

#[cfg(test)]
mod eq_model {
    use crate::models::EQProfile;

    use super::*;

    #[test]
    fn from_signed_check() {
        let signed_values = [-120, -100, -80, -60, -40, -20, 0, 20, 40, 60, 80, 100, 120];
        let expected_values = vec![0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240];
        let actual_values: Vec<u8> = signed_values.iter().map(MonoEQ::from_signed).collect();
        assert_eq!(expected_values, actual_values);
    }

    #[test]
    fn drc_transformation_check() {
        let initial_floats = vec![
            6.0, -6.0, 2.6000004, -3.0, 2.8000002, -1.6999998, 2.1999998, 0.39999962,
        ];

        // Extracted values from the original implementation
        let expected_post_drc_floats = vec![
            1.19351225,
            -1.61997,
            1.23241205,
            -1.0682,
            0.9448091,
            -0.735985,
            0.58319497,
            -0.13796605,
            0.0,
            -12.0,
        ];

        let eq = MonoEQ::calculate_drc_adjustments(initial_floats);
        assert_eq!(expected_post_drc_floats, eq);
    }

    #[test]
    fn drc_acoustic_bytes_check() {
        let expected_bytes = vec![125, 118, 123, 120, 124, 122, 124, 121, 120, 0];
        let eq = EQProfile::Acoustic.eq().to_drc_bytes();
        assert_eq!(expected_bytes, eq);
    }
}
