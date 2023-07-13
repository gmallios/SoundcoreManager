use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct BLEConnectionUuidSet {
    service_uuid: String,
    read_uuid: String,
    write_uuid: String,
}

#[wasm_bindgen]
impl BLEConnectionUuidSet {
    #[wasm_bindgen(constructor)]
    pub fn new(service_uuid: String, read_uuid: String, write_uuid: String) -> Self {
        BLEConnectionUuidSet {
            service_uuid,
            read_uuid,
            write_uuid,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn service_uuid(&self) -> String {
        self.service_uuid.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn read_uuid(&self) -> String {
        self.read_uuid.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn write_uuid(&self) -> String {
        self.write_uuid.clone()
    }
}

impl From<JsValue> for BLEConnectionUuidSet {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}
