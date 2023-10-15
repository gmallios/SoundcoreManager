use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct TouchTone(pub bool);

impl From<bool> for TouchTone {
    fn from(b: bool) -> Self {
        TouchTone(b)
    }
}
