use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, FromRepr};

use super::MonoEQ;

#[repr(u16)]
#[derive(
    FromRepr,
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
    EnumString,
    Default,
)]
pub enum EQProfile {
    // Regular profiles
    #[default]
    SoundcoreSignature = 0x0000,
    Acoustic = 0x0001,
    BassBooster = 0x0002,
    BassReducer = 0x0003,
    Classical = 0x0004,
    Podcast = 0x0005,
    Dance = 0x0006,
    Deep = 0x0007,
    Electronic = 0x0008,
    Flat = 0x0009,
    HipHop = 0x000A,
    Jazz = 0x000B,
    Latin = 0x000C,
    Lounge = 0x000D,
    Piano = 0x000E,
    Pop = 0x000F,
    RnB = 0x0010,
    Rock = 0x0011,
    SmallSpeakers = 0x0012,
    SpokenWord = 0x0013,
    TrebleBooster = 0x0014,
    TrebleReducer = 0x0015,
    Custom = 0xFEFE,
    // Professional profiles
    Foxes = 0x00EE,
    Halestorm = 0x01EE,
    Lecrae = 0x02EE,
    Daya = 0x03EE,
    CedricGervais = 0x04EE,
    TheInfamousStringdusters = 0x05EE,
    JohnPaulWhite = 0x06EE,
}

impl EQProfile {
    pub fn id(&self) -> u16 {
        *self as u16
    }

    pub fn from_id(id: u16) -> Option<Self> {
        Self::from_repr(id.to_be())
    }

    pub fn eq(&self) -> MonoEQ {
        let eq: [i8; 8] = match self {
            Self::SoundcoreSignature => [0, 0, 0, 0, 0, 0, 0, 0],
            Self::Acoustic => [40, 10, 20, 20, 40, 40, 40, 20],
            Self::BassBooster => [40, 30, 10, 0, 0, 0, 0, 0],
            Self::BassReducer => [-40, -30, -10, 0, 0, 0, 0, 0],
            Self::Classical => [30, 30, -20, -20, 0, 20, 30, 40],
            Self::Podcast => [-30, 20, 40, 40, 30, 20, 0, -20],
            Self::Dance => [20, -30, -10, 10, 20, 20, 10, -30],
            Self::Deep => [20, 10, 30, 30, 20, -20, -40, -50],
            Self::Electronic => [30, 20, -20, 20, 10, 20, 30, 30],
            Self::Flat => [-20, -20, -10, 0, 0, 0, -20, -20],
            Self::HipHop => [20, 30, -10, -10, 20, -10, 20, 30],
            Self::Jazz => [20, 20, -20, -20, 0, 20, 30, 40],
            Self::Latin => [0, 0, -20, -20, -20, 0, 30, 50],
            Self::Lounge => [-10, 20, 40, 30, 0, -20, 20, 10],
            Self::Piano => [0, 30, 30, 20, 40, 50, 30, 40],
            Self::Pop => [-10, 10, 30, 30, 10, -10, -20, -30],
            Self::RnB => [60, 20, -20, -20, 20, 30, 30, 40],
            Self::Rock => [30, 20, -10, -10, 10, 30, 30, 30],
            Self::SmallSpeakers => [40, 30, 10, 0, -20, -30, -40, -40],
            Self::SpokenWord => [-30, -20, 10, 20, 20, 10, 0, -30],
            Self::TrebleBooster => [-20, -20, -20, -10, 10, 20, 20, 40],
            Self::TrebleReducer => [0, 0, 0, -20, -30, -40, -40, -60],
            _ => [0, 0, 0, 0, 0, 0, 0, 0],
        };
        MonoEQ::from_signed_bytes(eq.into())
    }
}
