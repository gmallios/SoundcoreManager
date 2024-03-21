use soundcore_lib::{packets::ResponsePacket, types::SupportedModels};

#[test]
fn should_parse_state_update() {
    let packet = ResponsePacket::from_bytes(&test_data::a3040::anc);
    match packet {
        Ok(ResponsePacket::DeviceState(state)) => {
            // TODO: Assert state
            assert_eq!(state.tag, SupportedModels::A3040);
            println!("{:?}", state.data);
        }
        Err(err) => panic!("Failed to parse state update packet"),
        _ => panic!("Parsed as wrong packet type"),
    }
}

#[test]
fn should_parse_sound_mode_update_packet() {
    // Normal
    // {
    //     let packet = ResponsePacket::from_bytes(&test_data::a3040::SOUND_MODE_UPDATE_NORMAL);
    //     match packet {
    //         Ok(ResponsePacket::SoundModeUpdate(state)) => {
    //             assert_eq!(state.0.current, CurrentSoundMode::Normal);
    //         }
    //         Err(_err) => panic!("Failed to parse state update packet"),
    //         _ => panic!("Parsed as wrong packet type"),
    //     }
    // }
    // // Noise Cancelling
    // {
    //     let packet = ResponsePacket::from_bytes(&test_data::a3040::SOUND_MODE_UPDATE_NOISE_CANCELLING);
    //     match packet {
    //         Ok(ResponsePacket::SoundModeUpdate(state)) => {
    //             todo!()
    //         }
    //         Err(_err) => panic!("Failed to parse state update packet"),
    //         _ => panic!("Parsed as wrong packet type"),
    //     }
    // }
    // // Transparency
    // {
    //     let packet = ResponsePacket::from_bytes(&test_data::a3040::SOUND_MODE_UPDATE_TRANSPARENCY);
    //     match packet {
    //         Ok(ResponsePacket::SoundModeUpdate(state)) => {
    //             todo!()
    //         }
    //         Err(_err) => panic!("Failed to parse state update packet"),
    //         _ => panic!("Parsed as wrong packet type"),
    //     }
    // }
}
