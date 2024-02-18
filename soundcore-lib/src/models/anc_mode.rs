use serde::{Deserialize, Serialize};

use crate::models::AdaptiveANCMode;
use crate::models::SceneBasedANCMode;

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
Hash
)]
#[serde(rename_all = "camelCase")]
pub enum ANCMode {
    SceneBased(SceneBasedANCMode),
    Adaptive(AdaptiveANCMode),
}


impl ANCMode {
    pub fn as_u8(&self) -> u8 {
        match self {
            ANCMode::SceneBased(mode) => mode.as_u8(),
            ANCMode::Adaptive(mode) => mode.as_u8(),
        }
    }
    
    pub fn from_u8_scene_based(value: u8) -> Option<Self> {
        SceneBasedANCMode::from_u8(value).map(ANCMode::SceneBased)
    }
    
    pub fn from_u8_adaptive(value: u8) -> Option<Self> {
        AdaptiveANCMode::from_u8(value).map(ANCMode::Adaptive)
    }

    pub fn new_scene_based(mode: SceneBasedANCMode) -> Self {
        ANCMode::SceneBased(mode)
    }

    pub fn new_adaptive(mode: AdaptiveANCMode) -> Self {
        ANCMode::Adaptive(mode)
    }
}

impl Default for ANCMode {
    fn default() -> Self {
        ANCMode::SceneBased(Default::default())
    }
}


