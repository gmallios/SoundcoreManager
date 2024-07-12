use std::cell::RefCell;

use js_sys::{Array, Uint8Array};
use tokio::sync::mpsc::Receiver;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    BluetoothDevice, BluetoothRemoteGattCharacteristic, BluetoothRemoteGattServer,
    BluetoothRemoteGattService,
};

use manager_fut::ManagerFuture;
use soundcore_lib::ble::{BLEConnection, BLEDeviceDescriptor, WriteType};
use soundcore_lib::error::{SoundcoreLibError, SoundcoreLibResult};

pub struct WebBLEConnection {
    gatt_server: BluetoothRemoteGattServer,
    read_characteristic: BluetoothRemoteGattCharacteristic,
    write_characteristic: BluetoothRemoteGattCharacteristic,
    on_packet_handler: RefCell<Option<Closure<dyn Fn()>>>,
}

impl WebBLEConnection {
    pub async fn new(device: BluetoothDevice) -> Result<Self, JsValue> {
        let gatt = device.gatt().ok_or("GATT is not supported")?;
        let gatt_server: BluetoothRemoteGattServer = JsFuture::from(gatt.connect()).await?.into();
        let services: Array = JsFuture::from(gatt_server.get_primary_services())
            .await?
            .into();

        if services.length() == 0 {
            return Err(JsValue::from("0 services were returned"));
        }

        let service: BluetoothRemoteGattService = services.get(0).into();

        let characteristics: Array = JsFuture::from(service.get_characteristics()).await?.into();

        if characteristics.length() == 0 {
            return Err(JsValue::from("0 characteristics were returned"));
        }

        // TODO: Change this
        let write_characteristic: BluetoothRemoteGattCharacteristic = characteristics.get(0).into();
        let read_characteristic: BluetoothRemoteGattCharacteristic = characteristics.get(1).into();

        JsFuture::from(read_characteristic.start_notifications()).await?;

        Ok(Self {
            gatt_server,
            read_characteristic,
            write_characteristic,
            on_packet_handler: Default::default(),
        })
    }
}

impl BLEConnection for WebBLEConnection {
    fn descriptor(&self) -> BLEDeviceDescriptor {
        BLEDeviceDescriptor {
            addr: Default::default(),
            name: self.gatt_server.device().name().unwrap_or(String::default())
        }
    }
    async fn byte_channel(&self) -> SoundcoreLibResult<Receiver<Vec<u8>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(255);
        let read_characteristic = self.read_characteristic.clone();

        let on_packet_handler = Closure::new(move || {
            if let Some(value) = read_characteristic.value() {
                let data = Uint8Array::new(&value.buffer().into()).to_vec();
                let local_tx = tx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    local_tx.send(data).await.unwrap();
                });
            }
        });

        *self.on_packet_handler.borrow_mut() = Some(on_packet_handler);
        self.read_characteristic
            .set_oncharacteristicvaluechanged(Some(
                self.on_packet_handler
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            ));
        Ok(rx)
    }
    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()> {
        let mut data = bytes.to_vec();

        let promise = match write_type {
            WriteType::WithoutResponse => self
                .write_characteristic
                .write_value_without_response_with_u8_array(&mut data),
            WriteType::WithResponse => self
                .write_characteristic
                .write_value_with_response_with_u8_array(&mut data),
        };

        JsFuture::from(promise)
            .await
            .map_err(|e| SoundcoreLibError::SendError)?;
        Ok(())
    }
}

impl Drop for WebBLEConnection {
    fn drop(&mut self) {
        self.read_characteristic
            .set_oncharacteristicvaluechanged(None);
        self.gatt_server.disconnect();
        web_sys::console::log_1(&"Connection dropped!".into());
    }
}
