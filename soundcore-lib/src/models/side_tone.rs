use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct SideTone(pub bool);

impl From<bool> for SideTone {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
