#![cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod connection;
mod utils;
mod web_ble_device;

#[wasm_bindgen(start)]
pub fn wasm_init() {
    console_error_panic_hook::set_once();
}
