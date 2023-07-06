use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

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
