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

/* We can use Generic Arg Infer "#![feature(generic_arg_infer)]" once https://github.com/rust-lang/rust/issues/85077 is stabilized */
/* This also could be dynamically be created, since the bytes match the command id bytes */
pub const PACKET_KIND_MAP: [(&[u8; 2], ResponsePacketKind); 10] = [
    (&[0xFF, 0xFF], ResponsePacketKind::Unknown),
    /* Updates */
    (&[0x01, 0x01], ResponsePacketKind::StateUpdate),
    (&[0x01, 0x03], ResponsePacketKind::BattLevelUpdate),
    (&[0x01, 0x04], ResponsePacketKind::BattChargingUpdate),
    (&[0x01, 0x05], ResponsePacketKind::InfoUpdate),
    (&[0x01, 0x7F], ResponsePacketKind::LDACUpdate),
    (&[0x06, 0x01], ResponsePacketKind::SoundModeUpdate),
    /* Acks */
    (&[0x06, 0x81], ResponsePacketKind::SetSoundModeAck),
    (&[0x02, 0x81], ResponsePacketKind::SetEqAck),
    (&[0x02, 0x83], ResponsePacketKind::SetEqDrcAck),
];
