use soundcore_lib::packets::ResponsePacket;
use soundcore_lib::types::KnownProductCodes;
use test_data::a3947::A3947_UNKNOWN_STATE_PACKET;

#[test]
pub fn parse_a3947_state_packet() {
    let packet = ResponsePacket::from_bytes(&A3947_UNKNOWN_STATE_PACKET).unwrap();
    match packet {
        ResponsePacket::DeviceState(resp) => {
            println!("{:?}", resp);
            assert_eq!(resp.tag, KnownProductCodes::A3947);
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}