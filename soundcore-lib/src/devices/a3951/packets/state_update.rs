use crate::api::{RequestPacket, ResponseStateUpdatePacket};

#[derive(Debug, PartialEq, Eq)]
pub struct StateUpdatePacketResponse {
    /* The data the StateUpdate Response holds (eq_idx, eq, etc) */
    pub wear_detection: bool,
    pub touch_tone: bool,
}

impl ResponseStateUpdatePacket for StateUpdatePacketResponse {
    fn from_bytes(bytes: &[u8]) -> Option<StateUpdatePacketResponse> {
        if bytes[5] != 0x01 && bytes[6] != 0x01 && bytes.len() > 93 {
            return None;
        }
        Some(StateUpdatePacketResponse {
            wear_detection: bytes[91] == 0x01,
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
        todo!()
    }
}

#[cfg(test)]
mod packet_tests {
    use crate::api::ResponseStateUpdatePacket;

    #[test]
    fn parse_valid_packet() {
        const BYTES: [u8; 97] = [
            0x9, 0xFF, 0x00, 0x00, 0x01, 0x01, 0x01, 0x61, 0x00, 0x00, 0x01, 0x04, 0x04, 0x00,
            0x01, 0xFE, 0xFE, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x63, 0x01, 0x54, 0x01, 0x66, 0x01, 0x54, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x01, 0x01, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04F,
        ];
        let packet = super::StateUpdatePacketResponse::from_bytes(&BYTES);
        assert_eq!(
            packet,
            Some(super::StateUpdatePacketResponse {
                wear_detection: true,
                touch_tone: false,
            })
        );
    }
}
