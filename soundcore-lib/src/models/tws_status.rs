use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
pub struct TwsStatus(pub bool);

impl From<bool> for TwsStatus {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
