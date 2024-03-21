use soundcore_lib::{packets::ResponsePacket, types::SupportedModels};
use test_data::a3951::A3951_STATE_UPDATE_BYTES;

#[test]
fn parse_a3951_state_update() {
    let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES).unwrap();
    match packet {
        ResponsePacket::DeviceState(state) => {
            // TODO: Assert state
            assert_eq!(state.tag, SupportedModels::A3951);
            println!("{:?}", state.data);
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}
