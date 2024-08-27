use std::str::FromStr;
use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::models::EQProfile;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(typescript_custom_section)]
const TS_CUSTOM_TYPES: &'static str = r#"
type BLECompanyIdentifiers = number[][];
type BLEServiceUuids = string[];
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BLECompanyIdentifiers")]
    pub type BLECompanyIdentifiers;

    #[wasm_bindgen(typescript_type = "BLEServiceUuids")]
    pub type BLEServiceUuids;
}

#[wasm_bindgen]
pub fn generate_soundcore_service_uuids() -> Result<BLEServiceUuids, JsValue> {
    const PREFIX: &str = "01";
    const POSTFIX: &str = "f5da-0000-1000-8000-00805f9b34fb";
    const MIN: u8 = 0x00;
    const MAX: u8 = 0xFF;

    let mut uuids = Vec::with_capacity(MAX as usize);
    for i in MIN..=MAX {
        let uuid = format!("{}{}{}", PREFIX, format!("{:02x}", i), POSTFIX);
        if let Ok(uuid) = Uuid::parse_str(&uuid) {
            uuids.push(uuid.to_string());
        }
    }

    Ok(serde_wasm_bindgen::to_value(&uuids)?.into())
}

#[wasm_bindgen(js_name = "getSoundcoreMacPrefixes")]
pub fn get_soundcore_mac_prefixes() -> Result<BLECompanyIdentifiers, JsValue> {
    let prefixes = BluetoothAdrr::SOUNDCORE_MAC_PREFIXES
        .iter()
        .map(|prefix| prefix.to_vec())
        .collect::<Vec<Vec<u8>>>();
    Ok(serde_wasm_bindgen::to_value(&prefixes)?.into())
}

#[wasm_bindgen(js_name = "getPresetEqValue")]
pub fn get_preset_eq_value(profile: String, bands: usize) -> Result<Vec<u8>, JsValue> {
    Ok(EQProfile::from_str(&profile)
        .map_err(|err| format!("{err:?}"))?
        .eq()
        .to_bytes(bands))
}
