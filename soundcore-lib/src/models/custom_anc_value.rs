use log::trace;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(
Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
#[typeshare]
pub struct CustomANCValue(pub u8);

impl CustomANCValue {
    pub fn from_u8(value: u8) -> Self {
        trace!("CustomANC::from_u8({})", value);
        match value {
            255 => CustomANCValue(255),
            // TODO: Check if any other device has a different range and implement it
            _ => CustomANCValue(value.clamp(0, 5)),
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
