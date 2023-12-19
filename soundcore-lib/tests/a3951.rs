use soundcore_lib::packets::ResponsePacket;
use test_data::a3951::A3951_STATE_UPDATE_BYTES;

#[test]
fn parse_a3951_state_update() {
    let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES).unwrap();
    match packet {
        ResponsePacket::DeviceState(_state) => {
            // TODO
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}
