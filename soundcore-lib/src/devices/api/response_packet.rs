use super::state_update::{ResponseStateUpdatePacket, ResponseStateUpdatePackets};
use crate::devices::SupportedModelIDs;

// Response Packet Structure
// | Byte  | Description              |
// |-------|--------------------------|
// |   0   | Prefix byte 1 (0x09)     |
// |   1   | Prefix byte 2 (0xFF)     |
// |   2   | Prefix byte 3 (0x00)     |
// |   3   | Prefix byte 4 (0x00)     |
// |   4   | Prefix byte 5 (0x01)     |
// |   5   | Command ID byte 1 (0x01) |
// |   6   | Command ID byte 2 (0x04) |
// |  ...  | Other data bytes         |
pub(crate) const RESPONSE_PREFIX: [u8; 5] = [0x09, 0xff, 0x00, 0x00, 0x01];

#[derive(Debug, PartialEq, Eq)]
pub enum ResponsePacket {
    StateUpdate(ResponseStateUpdatePackets),
}

impl ResponsePacket {
    fn from_bytes(device: SupportedModelIDs, bytes: &[u8]) -> Option<ResponsePacket> {
        if !bytes.starts_with(&RESPONSE_PREFIX) {
            return None;
        }

        match device {
            SupportedModelIDs::A3951 => {
                let packet = crate::devices::a3951::packets::state_update::StateUpdatePacketResponse::from_bytes(bytes);
                packet.map(|packet| {
                    ResponsePacket::StateUpdate(ResponseStateUpdatePackets::A3951(packet))
                })
            }
            SupportedModelIDs::A3027 => {
                todo!()
            }
        }
    }
}

// fn get() {
//     let packet = ResponsePacket::from_bytes(SupportedDevices::A3951, &[0x00]);
//     match packet {
//         Some(packet) => {
//             match packet {
//                 ResponsePacket::StateUpdate(state_update_packet) => {
//                     match state_update_packet {
//                         ResponseStateUpdatePackets::A3951(_a3951_state_update_packet) => {
//                             // Do something with a3951_state_update_packet
//                         }
//                     }
//                 }
//             }
//         }
//         None => {
//             // Do something
//         }
//     }
// }

#[cfg(test)]
mod response_tests {
    use super::ResponsePacket;
    use crate::devices::SupportedModelIDs;

    #[test]
    fn return_none_when_none_match() {
        let packet = ResponsePacket::from_bytes(SupportedModelIDs::A3951, &[0x00]);
        assert_eq!(packet, None);
    }

    #[test]
    fn handle_wrong_prefix() {
        let packet = ResponsePacket::from_bytes(SupportedModelIDs::A3027, &[0x09, 0xfd]);
        assert_eq!(packet, None);
    }
}
