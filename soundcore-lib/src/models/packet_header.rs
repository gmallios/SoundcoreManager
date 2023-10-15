use serde::{Deserialize, Serialize};

use super::ResponsePacketKind;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResponsePacketHeader {
    pub kind: ResponsePacketKind,
    pub length: u16,
}
