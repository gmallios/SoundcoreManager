use std::str::FromStr;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod ble_uuid;
mod request_packet;
mod response_packets;
mod state;

#[wasm_bindgen(typescript_custom_section)]
const MAC_ADDRESS_PREFIXES: &'static str = r#"
type MacAddressPrefixes = number[][]
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MacAddressPrefixes")]
    pub type MacAddressPrefixes;
}

#[wasm_bindgen(js_name = "getSoundcoreMacPrefixes")]
pub fn get_soundcore_mac_prefixes() -> Result<MacAddressPrefixes, JsValue> {
    let prefixes = soundcore_lib::mac::SOUNDCORE_MAC_PREFIXES
        .iter()
        .map(|prefix| prefix.to_vec())
        .collect::<Vec<Vec<u8>>>();
    Ok(serde_wasm_bindgen::to_value(&prefixes)?.into())
}

#[wasm_bindgen(js_name = "getUUIDSet")]
pub fn get_uuid_set(model: &str) -> Result<ble_uuid::BLEConnectionUuidSet, JsValue> {
    let model = soundcore_lib::devices::SupportedModelIDs::from_str(model);
    if let Ok(model) = model {
        let uuids = soundcore_lib::devices::match_model_id_to_uuid_set(&model);
        Ok(serde_wasm_bindgen::to_value(&uuids)?.into())
    } else {
        Err(JsValue::from_str("Invalid model"))
    }
}

#[wasm_bindgen(js_name = "matchNameToModelID")]
pub fn match_name_to_model_id(name: &str) -> Result<String, JsValue> {
    let model = soundcore_lib::devices::match_name_to_model_id(name);
    if let Some(model) = model {
        Ok(model.to_string())
    } else {
        Err(JsValue::from_str("Invalid model"))
    }
}

/// Returns a map of all supported model IDs to their respective UUID sets.
#[wasm_bindgen(js_name = "getAllUUIDSets")]
pub fn get_all_uuid_sets() -> Result<JsValue, JsValue> {
    let uuid_sets = soundcore_lib::devices::get_all_uuid_sets();
    Ok(serde_wasm_bindgen::to_value(&uuid_sets)?.into())
}
