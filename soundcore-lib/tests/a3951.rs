use soundcore_lib::packets::ResponsePacket;

const STATE_UPDATE_BYTES: [u8; 97] = [
    9, 255, 0, 0, 1, 1, 1, 97, 0, 1, 1, 5, 5, 1, 0, 254, 254, 160, 150, 130, 120, 120, 120, 120,
    120, 160, 150, 130, 120, 120, 120, 120, 120, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99, 1, 84, 1, 102, 1, 84, 0, 1, 0, 0, 0,
    1, 1, 6, 0, 1, 0, 0, 0, 0, 242,
];

#[test]
fn parse_a3951_state_update() {
    let packet = ResponsePacket::from_bytes(&STATE_UPDATE_BYTES).unwrap();
    match packet {
        ResponsePacket::DeviceState(_state) => {
            todo!()
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}
