use serde::{Deserialize, Serialize};

use crate::error::SoundcoreError;

pub type SendFnType<'a> = &'a (dyn Fn(&[u8]) -> Result<(), SoundcoreError> + Send + Sync);
pub type RecvFnType<'a> = &'a (dyn Fn(usize) -> Result<Vec<u8>, SoundcoreError> + Send + Sync);
pub type CloseSockFnType<'a> = &'a (dyn Fn() + Send + Sync);

pub trait ResponseDecoder<T> {
    fn decode(arr: &[u8]) -> Result<T, SoundcoreError>;
}

pub trait CommandEncoder<T> {
    fn encode(&self) -> Result<Vec<u8>, SoundcoreError>;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub left_fw: String,
    pub right_fw: String,
    pub sn: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery_level: BatteryLevel,
    pub battery_charging: BatteryCharging,
    pub anc_status: ANCProfile,
    pub side_tone_enabled: bool,
    pub wear_detection_enabled: bool,
    pub touch_tone_enabled: bool,
    pub left_eq: EQWave,
    pub right_eq: EQWave,
    pub hearid_enabled: bool,
    pub left_hearid: EQWave,
    pub right_hearid: EQWave,
    pub left_hearid_customdata: EQWave,
    pub right_hearid_customdata: EQWave,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct BatteryCharging {
    pub left: bool,
    pub right: bool,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct BatteryLevel {
    pub left: u8,
    pub right: u8,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct ANCProfile {
    pub option: u8,
    pub anc_option: u8,
    pub transparency_option: u8,
    pub anc_custom: u8,
}

/* This gets received from the device and is used to create the EQ to send. */
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct EQWave {
    pub pos0: f32,
    pub pos1: f32,
    pub pos2: f32,
    pub pos3: f32,
    pub pos4: f32,
    pub pos5: f32,
    pub pos6: f32,
    pub pos7: f32,
    pub pos8: f32,
    pub pos9: f32,
}

impl EQWave {
    pub const HEARD_ID_DEFAULT: EQWave = EQWave {
        pos0: 25.5,
        pos1: 25.5,
        pos2: 25.5,
        pos3: 25.5,
        pos4: 25.5,
        pos5: 25.5,
        pos6: 25.5,
        pos7: 25.5,
        pos8: 25.5,
        pos9: 25.5,
    };

    pub fn decode(arr: &[u8]) -> Result<EQWave, SoundcoreError> {
        if arr.len() < 8 {
            return Err(SoundcoreError::Unknown);
        }

        let results = Self::eq_int_to_float(arr);
        Ok(EQWave {
            pos0: arr[0] as f32 / 10.0, //6.0 - 18.0
            pos1: results[1],
            pos2: results[2],
            pos3: results[3],
            pos4: results[4],
            pos5: results[5],
            pos6: results[6],
            pos7: results[7],
            /* Since A3951 uses 8-band eq these are constant */
            pos8: 12.0,
            pos9: 0.0,
        })
    }

    fn eq_int_to_float(arr: &[u8]) -> Vec<f32> {
        let mut eq: Vec<f32> = Vec::new();
        let max_val: f32 = (12.0 + 7.0) - 1.0;
        let min_val: f32 = (12.0 - 7.0) + 1.0;
        for i in arr {
            let f: f32 = *i as f32 / 10.0;
            if f > max_val {
                eq.push(max_val);
            } else if f < min_val {
                eq.push(min_val);
            } else {
                eq.push(f);
            }
        }
        eq
    }

    /* A3951 "Needs" drc, other devices might not (see m10061y0 in jadx) */
    pub fn transform_to_realeq(input_wave: EQWave) -> EQWave {
        Self::transform_addsub(
            Self::apply_drc(Self::transform_addsub(input_wave, false, 12.0)),
            true,
            12.0,
        )
    }

    fn apply_drc(mut input_wave: EQWave) -> EQWave {
        /* Spaghetti code, ported straight from Soundcore App */
        const EQCONST_A: f32 = 0.85;
        const EQCONST_B: f32 = 0.95;
        let (d, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12) = (
            input_wave.pos0 as f64,
            input_wave.pos1 as f64,
            EQCONST_A as f64,
            input_wave.pos2 as f64,
            input_wave.pos3 as f64,
            input_wave.pos4 as f64,
            input_wave.pos5 as f64,
            input_wave.pos6 as f64,
            input_wave.pos7 as f64,
            EQCONST_B as f64,
            (input_wave.pos2 * 0.81 * EQCONST_A) as f64,
            (input_wave.pos5 * 0.81 * EQCONST_A) as f64,
        );
        input_wave.pos0 = ((((((((1.26 * d) - ((d2 * 0.71) * d3)) + (d4 * 0.177))
            - (d5 * 0.0494))
            + (d6 * 0.0345))
            - (d7 * 0.0197))
            + (d8 * 0.0075))
            - (0.00217 * d9)) as f32;
        input_wave.pos1 = ((((((((((-0.71) * d) * d3) + ((d2 * 1.73) * d10)) - d11)
            + (d5 * 0.204))
            - (d6 * 0.068))
            + (d7 * 0.045))
            - (d8 * 0.0235))
            + (d9 * 0.0075)) as f32;
        input_wave.pos2 = ((((((((d * 0.177) - ((d2 * 0.81) * d3)) + ((d4 * 1.73) * d10))
            - ((d5 * 0.81) * d3))
            + (d6 * 0.208))
            - (d7 * 0.07))
            + (d8 * 0.045))
            - (d9 * 0.0197)) as f32;
        input_wave.pos3 = (((((((((-0.0494) * d) + (d2 * 0.204)) - d11) + ((d5 * 1.73) * d10))
            - ((d6 * 0.82) * d3))
            + (d7 * 0.208))
            - (d8 * 0.068))
            + (d9 * 0.0345)) as f32;
        input_wave.pos4 = ((((((((d * 0.0345) - (d2 * 0.068)) + (d4 * 0.208))
            - ((0.82 * d5) * d3))
            + ((d6 * 1.73) * d10))
            - d12)
            + (d8 * 0.204))
            - (d9 * 0.0494)) as f32;
        input_wave.pos5 = (((((((((-0.0197) * d) + (d2 * 0.045)) - (0.07 * d4)) + (0.208 * d5))
            - ((d6 * 0.81) * d3))
            + ((1.73 * d7) * d10))
            - ((0.81 * d8) * d3))
            + (d9 * 0.177)) as f32;
        input_wave.pos6 = ((((((((d * 0.0075) - (d2 * 0.0235)) + (0.045 * d4)) - (d5 * 0.068))
            + (0.204 * d6))
            - d12)
            + ((1.83 * d8) * d10))
            - ((d9 * 0.71) * d3)) as f32;
        input_wave.pos7 = ((((((((d * (-0.00217)) + (d2 * 0.0075)) - (d4 * 0.0197))
            + (d5 * 0.0345))
            - (d6 * 0.0494))
            + (d7 * 0.177))
            - ((d8 * 0.71) * d3))
            + (d9 * 1.5)) as f32;
        Self::transform_multdiv(input_wave, false, 10.0)
    }

    fn transform_multdiv(mut input_wave: EQWave, should_mult: bool, factor: f32) -> EQWave {
        if should_mult {
            input_wave.pos0 *= factor;
            input_wave.pos1 *= factor;
            input_wave.pos2 *= factor;
            input_wave.pos3 *= factor;
            input_wave.pos4 *= factor;
            input_wave.pos5 *= factor;
            input_wave.pos6 *= factor;
            input_wave.pos7 *= factor;
        } else {
            input_wave.pos0 /= factor;
            input_wave.pos1 /= factor;
            input_wave.pos2 /= factor;
            input_wave.pos3 /= factor;
            input_wave.pos4 /= factor;
            input_wave.pos5 /= factor;
            input_wave.pos6 /= factor;
            input_wave.pos7 /= factor;
        }
        input_wave
    }

    fn transform_addsub(mut input_wave: EQWave, should_add: bool, offset: f32) -> EQWave {
        if should_add {
            input_wave.pos0 += offset;
            input_wave.pos1 += offset;
            input_wave.pos2 += offset;
            input_wave.pos3 += offset;
            input_wave.pos4 += offset;
            input_wave.pos5 += offset;
            input_wave.pos6 += offset;
            input_wave.pos7 += offset;
        } else {
            input_wave.pos0 -= offset;
            input_wave.pos1 -= offset;
            input_wave.pos2 -= offset;
            input_wave.pos3 -= offset;
            input_wave.pos4 -= offset;
            input_wave.pos5 -= offset;
            input_wave.pos6 -= offset;
            input_wave.pos7 -= offset;
        }
        input_wave
    }
}

/* This gets sent to the device. EQWave is transformed into this. */
#[derive(Default, Debug)]
pub struct EQWaveInt {
    pos0: i16,
    pos1: i16,
    pos2: i16,
    pos3: i16,
    pos4: i16,
    pos5: i16,
    pos6: i16,
    pos7: i16,
    pos8: i16,
    pos9: i16,
}

impl EQWaveInt {
    pub fn from_eq_wave(eq: EQWave) -> EQWaveInt {
        const F: f32 = 10.0; /* Constant derived from method usage in the Soundcore App */
        EQWaveInt {
            pos0: (eq.pos0 * F).round() as i16,
            pos1: (eq.pos1 * F).round() as i16,
            pos2: (eq.pos2 * F).round() as i16,
            pos3: (eq.pos3 * F).round() as i16,
            pos4: (eq.pos4 * F).round() as i16,
            pos5: (eq.pos5 * F).round() as i16,
            pos6: (eq.pos6 * F).round() as i16,
            pos7: (eq.pos7 * F).round() as i16,
            pos8: (eq.pos8 * F).round() as i16,
            pos9: (eq.pos9 * F).round() as i16,
        }
    }

    pub fn to_bytes(&self) -> [u8; 10] {
        [
            (self.pos0 as u8),
            (self.pos1 as u8),
            (self.pos2 as u8),
            (self.pos3 as u8),
            (self.pos4 as u8),
            (self.pos5 as u8),
            (self.pos6 as u8),
            (self.pos7 as u8),
            (self.pos8 as u8),
            (self.pos9 as u8),
        ]
    }

    pub fn to_8bytes(&self) -> [u8; 8] {
        [
            (self.pos0 as u8),
            (self.pos1 as u8),
            (self.pos2 as u8),
            (self.pos3 as u8),
            (self.pos4 as u8),
            (self.pos5 as u8),
            (self.pos6 as u8),
            (self.pos7 as u8),
        ]
    }
}
