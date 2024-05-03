use soundcore_lib::{
    models::{EQConfiguration, EQProfile},
    packets::ResponsePacket,
    types::KnownProductCodes,
};
use test_data::a3951::{A3951_STATE_UPDATE_BYTES, A3951_STATE_UPDATE_BYTES_2};

#[test]
fn parse_a3951_state_update() {
    let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES).unwrap();
    match packet {
        ResponsePacket::DeviceState(resp) => {
            // TODO: Assert state
            assert_eq!(resp.tag, KnownProductCodes::A3951);
            if let EQConfiguration::Stereo(stereo) = resp.data.eq {
                assert_eq!(stereo.profile, EQProfile::Custom);
            } else {
                panic!("Parsed as wrong EQConfiguration type");
            }
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}

#[test]
fn parse_a3951_state_update_2() {
    let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES_2).unwrap();
    match packet {
        ResponsePacket::DeviceState(resp) => {
            assert_eq!(resp.tag, KnownProductCodes::A3951);
            if let EQConfiguration::Stereo(stereo) = resp.data.eq {
                assert_eq!(stereo.profile, EQProfile::JohnPaulWhite);
            } else {
                panic!("Parsed as wrong EQConfiguration type");
            }
        }
        _ => panic!("Parsed as wrong packet type"),
    }
}
