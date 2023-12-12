
use serde::{Serialize, Deserialize};
use strum::FromRepr;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, FromRepr,
)]
pub enum PromptLanguage {
    English = 0x00,
    Chinese = 0x01,
}