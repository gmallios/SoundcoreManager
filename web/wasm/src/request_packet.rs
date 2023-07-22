use std::str::FromStr;

use soundcore_lib::{api::RequestPacket, devices::SupportedModelIDs};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::state::{EQValues, SoundMode};

/// Returns a packet for the given model ID if one exists.
#[wasm_bindgen]
pub fn get_state_update_packet(modeid: &str) -> Result<Vec<u8>, String> {
    let model = soundcore_lib::devices::SupportedModelIDs::from_str(modeid)
        .map_err(|e| format!("Invalid model: {}", e))?;

    match model {
        SupportedModelIDs::A3951 => Ok(
            soundcore_lib::devices::a3951::packets::state_update::StateUpdateRequestPacket::new()
                .bytes(),
        ),
        _ => Err(format!("No state update packet found for model {}", modeid)),
    }
}

#[wasm_bindgen]
pub fn get_set_eq_packet(modeid: &str, new_eq: EQValues) -> Result<Vec<u8>, String> {
    todo!()
}

#[wasm_bindgen]
pub fn get_set_sound_mode_packet(modeid: &str, new_mode: SoundMode) -> Result<Vec<u8>, String> {
    todo!()
}


#[cfg(test)]
mod request_packet_tests {

    #[test]
    fn should_return_packet_for_a3951() {
        let packet = super::get_state_update_packet("A3951");
        assert!(packet.is_ok());
    }
}
