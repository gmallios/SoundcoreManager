use std::sync::Arc;

use js_sys::Function;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::BluetoothDevice;

use manager_fut::{ManagerFuture, WasmFuture};
use soundcore_lib::device::SoundcoreBLEDevice;

use crate::connection::WebBLEConnection;

/// Thin wrapper over the actual Device implementation
/// which translates calls between Rust and JS-land
#[wasm_bindgen]
pub struct WebBLEDevice {
    device: SoundcoreBLEDevice<WebBLEConnection, WasmFuture>,
}

#[wasm_bindgen]
impl WebBLEDevice {
    #[wasm_bindgen]
    pub async fn new(device: BluetoothDevice) -> Result<WebBLEDevice, JsValue> {
        let conn = WebBLEConnection::new(device).await?;
        let device = SoundcoreBLEDevice::new(Arc::new(conn))
            .await
            .map_err(|e| format!("{e:?}"))?;
        Ok(Self { device })
    }

    #[wasm_bindgen(js_name = "setOnStateChange")]
    pub async fn set_on_state_change(&self, cb: Function) {
        let mut rx = self.device.state_channel().await;
        wasm_bindgen_futures::spawn_local(async move {
            while let Ok(()) = rx.changed().await {
                web_sys::console::log_1(&"State changed!".into());
                let state = rx.borrow_and_update();
                let state_value = serde_wasm_bindgen::to_value(&*state)
                    .expect("The state should be convertible to JsValue");
                cb.call1(&JsValue::null(), &state_value)
                    .expect("JS should handle this error");
            }
            web_sys::console::log_1(&"Task dropped!".into());
        });
    }

    #[wasm_bindgen(js_name = "latestState")]
    pub async fn latest_state(&self) -> Result<SoundcoreDeviceState, JsValue> {
        let state = self.device.latest_state().await;
        Ok(serde_wasm_bindgen::to_value(&state)?.into())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SoundcoreDeviceState")]
    pub type SoundcoreDeviceState;
}
