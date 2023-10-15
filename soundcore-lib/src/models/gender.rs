use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct Gender(u8);

impl Gender {
    pub fn from_u8(value: u8) -> Self {
        Self(value)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
