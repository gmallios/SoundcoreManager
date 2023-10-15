use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct CustomANC(pub u8);

impl CustomANC {
    pub fn from_u8(value: u8) -> Self {
        match value {
            255 => CustomANC(255),
            _ => CustomANC(value.clamp(0, 10)),
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
