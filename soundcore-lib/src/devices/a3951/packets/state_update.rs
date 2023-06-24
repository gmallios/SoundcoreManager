use log::warn;

use crate::api::{
    BatteryLevel, ChargingStatus, EQValues, RequestPacket, ResponseStateUpdatePacket, SoundMode,
};

#[derive(Debug, PartialEq, Eq)]
pub struct StateUpdatePacketResponse {
    pub sound_mode: SoundMode,
    pub eq: (EQValues, EQValues),
    pub charging_status: ChargingStatus,
    pub battery_level: BatteryLevel,
    pub hearid_status: bool,
    pub hearid: (EQValues, EQValues),
    pub host_device: u8,
    pub tws_status: bool,
    pub wear_detection: bool,
    pub touch_tone: bool,
    pub side_tone: bool,
}

impl ResponseStateUpdatePacket for StateUpdatePacketResponse {
    fn from_bytes(bytes: &[u8]) -> Option<StateUpdatePacketResponse> {
        if bytes[5] != 0x01 && bytes[6] != 0x01 && bytes.len() > 93 {
            return None;
        }

        let Some(sound_mode) = SoundMode::from_bytes([bytes[86], bytes[87], bytes[88], bytes[89]]) else {
            warn!("Invalid sound mode: {:?}", [bytes[86], bytes[87], bytes[88], bytes[89]]);
            return None;
        };

        let left_eq = EQValues::from_bytes([
            bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22], bytes[23], bytes[24],
        ]);
        let right_eq = EQValues::from_bytes([
            bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30], bytes[31], bytes[32],
        ]);
        let hearid_left = EQValues::from_bytes([
            bytes[36], bytes[37], bytes[38], bytes[39], bytes[40], bytes[41], bytes[42], bytes[43],
        ]);
        let hearid_right = EQValues::from_bytes([
            bytes[44], bytes[45], bytes[46], bytes[47], bytes[48], bytes[49], bytes[50], bytes[51],
        ]);

        Some(StateUpdatePacketResponse {
            sound_mode,
            eq: (left_eq, right_eq),
            battery_level: BatteryLevel::from_bytes([bytes[11], bytes[12]]),
            charging_status: ChargingStatus::from_bytes([bytes[13], bytes[14]]),
            hearid: (hearid_left, hearid_right),
            host_device: bytes[9],
            tws_status: bytes[10] == 0x01,
            hearid_status: bytes[35] == 0x01,
            wear_detection: bytes[91] == 0x01,
            side_tone: bytes[90] == 0x01,
            touch_tone: bytes[92] == 0x01,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StateUpdateRequestPacket {}

impl StateUpdateRequestPacket {
    pub fn new() -> StateUpdateRequestPacket {
        StateUpdateRequestPacket {}
    }
}

impl RequestPacket for StateUpdateRequestPacket {
    fn bytes(&self) -> Vec<u8> {
        vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01, 0xA, 0x00, 0x02]
    }
}

#[cfg(test)]
mod packet_tests {

    #[test]
    fn parse_valid_packet() {
        // const BYTES: [u8; 97] = [
        //     0x9, 0xFF, 0x00, 0x00, 0x01, 0x01, 0x01, 0x61, 0x00, 0x00, 0x01, 0x04, 0x04, 0x00,
        //     0x01, 0xFE, 0xFE, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78,
        //     0x78, 0x78, 0x78, 0x78, 0x78, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        //     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        //     0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x63, 0x01, 0x54, 0x01, 0x66, 0x01, 0x54, 0x00, 0x01,
        //     0x00, 0x00, 0x00, 0x01, 0x01, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04F,
        // ];
        // let packet = super::StateUpdatePacketResponse::from_bytes(&BYTES);
        // assert_eq!(
        //     packet,
        //     Some(super::StateUpdatePacketResponse {
        //         wear_detection: true,
        //         touch_tone: false,
        //     })
        // );
    }
}
