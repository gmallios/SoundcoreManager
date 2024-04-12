use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
#[typeshare]
pub struct CustomTransparencyValue(pub u8);

impl CustomTransparencyValue {
    pub fn from_u8(value: u8) -> Self {
        match value {
            255 => CustomTransparencyValue(255),
            _ => CustomTransparencyValue(value.clamp(0, 10)),
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
