use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct WearDetection(pub bool);

impl From<bool> for WearDetection {
    fn from(b: bool) -> Self {
        WearDetection(b)
    }
}
