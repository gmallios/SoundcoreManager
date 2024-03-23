use serde::{Deserialize, Serialize};
use strum::FromRepr;
use typeshare::typeshare;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, FromRepr,
)]
#[repr(u8)]
#[typeshare]
pub enum PromptLanguage {
    English = 0x00,
    Chinese = 0x01,
}
