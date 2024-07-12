#![cfg(target_arch = "wasm32")]
use log::Level;
use wasm_bindgen::prelude::*;

mod connection;
mod utils;
mod web_ble_device;

#[wasm_bindgen(start)]
pub fn wasm_init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}
