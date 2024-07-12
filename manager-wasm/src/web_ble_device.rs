use std::sync::Arc;

use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::BluetoothDevice;

use manager_fut::{ManagerFuture, WasmFuture};
use soundcore_lib::device::SoundcoreBLEDevice;
use soundcore_lib::models::{EQConfiguration, MonoEQ, SoundMode};

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

    #[wasm_bindgen(js_name = "setSoundMode")]
    pub async fn set_sound_mode(&self, sound_mode: String) -> Result<(), JsValue> {
        let sound_mode: SoundMode =
            serde_json::from_str(&sound_mode).map_err(|err| format!("{err:?}"))?;
        self.device
            .set_sound_mode(sound_mode)
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "setEqualizerCustom")]
    pub async fn set_custom_eq(&self, bytes: &[i8]) -> Result<(), JsValue> {
        let eq = EQConfiguration::mono_custom(MonoEQ::from_signed_bytes(bytes.to_vec()));
        self.device
            .set_eq(eq)
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "setEqualizerPreset")]
    pub async fn set_preset_eq(&self, preset: String) -> Result<(), JsValue> {
        let eq = EQConfiguration::stereo_with_profile(
            serde_json::from_str(&preset).map_err(|err| format!("{err:?}"))?,
        );
        self.device
            .set_eq(eq)
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SoundcoreDeviceState")]
    pub type SoundcoreDeviceState;
}
