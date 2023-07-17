use std::str::FromStr;

use soundcore_lib::{
    api::ResponsePackets,
    devices::{a3951::transformers::packet_to_transformer, SupportedModelIDs},
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::state::SoundcoreDeviceState;

/// Takes in the current state, the model
/// and the current_state and returns the (Optionally) new state
#[wasm_bindgen]
pub fn handle_byte_response(
    bytes: &[u8],
    modelid: String,
    current_state: SoundcoreDeviceState,
) -> SoundcoreDeviceState {
    let modelid = SupportedModelIDs::from_str(&modelid).unwrap();
    let response_packet: Option<ResponsePackets> = ResponsePackets::from_bytes(modelid, bytes);
    match response_packet {
        Some(packet) => {
            if let Some(transformer) = packet_to_transformer(packet) {
                transformer.transform(&current_state.into()).into()
            } else {
                current_state
            }
        }
        None => current_state,
    }
}

#[cfg(test)]
mod response_tests {
    use crate::state::{BatteryLevel, ChargingStatus, EQValues, SoundMode, SoundcoreDeviceState};

    #[test]
    fn transforms_a9351_state() {
        let initial_state = SoundcoreDeviceState {
            sound_mode: SoundMode::new("NormalMode"),
            eq: EQValues::new(&[0, 0, 0, 0, 0, 0, 0, 0]),
            battery_level: Default::default(),
            charging_status: Default::default(),
        };

        let bytes = [
            9, 255, 0, 0, 1, 1, 1, 97, 0, 0, 1, 4, 4, 0, 1, 254, 254, 150, 149, 131, 125, 130, 131,
            137, 150, 150, 149, 131, 125, 130, 131, 137, 150, 255, 255, 0, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99, 1, 84, 1, 102,
            1, 84, 0, 1, 0, 0, 0, 1, 1, 6, 0, 1, 0, 0, 0, 0, 109,
        ];

        let modelid = "A3951";
        let new_state: SoundcoreDeviceState =
            super::handle_byte_response(&bytes, modelid.to_string(), initial_state);

        let expected_state: SoundcoreDeviceState = SoundcoreDeviceState {
            sound_mode: SoundMode::new("NoiseCancelling(Outdoor)"),
            eq: EQValues::new(&[30, 29, 11, 5, 10, 11, 17, 30]),
            battery_level: BatteryLevel { left: 4, right: 4 },
            charging_status: ChargingStatus {
                left: false,
                right: true,
            },
        };
        assert_eq!(new_state, expected_state);
    }
}
