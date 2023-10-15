use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ResponsePacketKind {
    /* Packets which *should* update the state in some way */
    StateUpdate,
    InfoUpdate,
    SoundModeUpdate,
    BattLevelUpdate,
    BattChargingUpdate,
    LDACUpdate,
    /* Acknowledgment packets */
    SetSoundModeAck,
    SetEqAck,
    SetEqDrcAck,
    Unknown,
}
