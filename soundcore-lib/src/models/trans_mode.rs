use log::trace;
use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};

use crate::models::{CustomizableTransparencyMode, NonCustomizableTransparencyMode};

#[repr(u8)]
#[derive(
Debug,
Serialize,
Deserialize,
Eq,
PartialEq,
Ord,
PartialOrd,
Clone,
Copy,
FromRepr,
Display,
Hash,
)]
#[serde(rename_all = "camelCase")]
pub enum TransparencyMode {
    NonCustomizable(NonCustomizableTransparencyMode),
    Customizable(CustomizableTransparencyMode),
}

impl TransparencyMode {
    pub fn as_u8(&self) -> u8 {
        match self {
            TransparencyMode::NonCustomizable(mode) => mode.as_u8(),
            TransparencyMode::Customizable(mode) => mode.as_u8(),
        }
    }
    
    pub fn from_u8_customizable(value: u8) -> Option<Self> {
        CustomizableTransparencyMode::from_u8(value).map(TransparencyMode::Customizable)
    }
    
    pub fn from_u8_non_customizable(value: u8) -> Option<Self> {
        NonCustomizableTransparencyMode::from_u8(value).map(TransparencyMode::NonCustomizable)
    }
}

impl Default for TransparencyMode {
    fn default() -> Self {
        TransparencyMode::NonCustomizable(Default::default())
    }
}
// #[cfg(test)]
// mod anc_mode_tests {
//     use super::*;
//
//     #[test]
//     fn init_from_u8() {
//         assert_eq!(
//             NonCustomizableTransparencyMode::from_u8(0),
//             Some(NonCustomizableTransparencyMode::FullyTransparent)
//         );
//         assert_eq!(NonCustomizableTransparencyMode::from_u8(1), Some(NonCustomizableTransparencyMode::Vocal));
//     }
//
//     #[test]
//     fn init_from_u8_invalid() {
//         assert_eq!(NonCustomizableTransparencyMode::from_u8(10), None);
//     }
//
//     #[test]
//     fn returns_value() {
//         assert_eq!(NonCustomizableTransparencyMode::FullyTransparent.as_u8(), 0);
//         assert_eq!(NonCustomizableTransparencyMode::Vocal.as_u8(), 1);
//     }
// }
