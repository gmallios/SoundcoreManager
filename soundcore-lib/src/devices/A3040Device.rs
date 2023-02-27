use bluetooth_lib::{BluetoothAdrr, platform::RFCOMM, RFCOMMClient};
use tokio::time::sleep;

use crate::{base::{SoundcoreDevice, SoundcoreHearID, SoundcoreANC, SoundcoreLDAC, SoundcoreEQ}, error::SoundcoreError, statics::A3040_RFCOMM_UUID, utils::remove_padding, types::{DeviceStatus, BatteryCharging, BatteryLevel, DeviceInfo}};

#[derive(Default)]
pub struct A3040 {
    btaddr: Option<BluetoothAdrr>,
    rfcomm: Option<RFCOMM>
}



#[async_trait::async_trait]
impl SoundcoreDevice for A3040 {
    async fn init(&self, btaddr: BluetoothAdrr) -> Result<Box<dyn SoundcoreDevice>, SoundcoreError> {
        let mut rfcomm = RFCOMM::new().await?;
        rfcomm
            .connect_uuid(btaddr.clone(), A3040_RFCOMM_UUID)
            .await?;
        Ok(Box::new(A3040 {
            btaddr: Some(btaddr),
            rfcomm: Some(rfcomm),
        }))
    }
    async fn close(&self) -> Result<(), SoundcoreError> {
        match &self.rfcomm {
            Some(rfcomm) => {
                rfcomm.close().await;
                Ok(())
            }
            None => Err(SoundcoreError::NotConnected),
        }
    }

    async fn send(&self, data: &[u8]) -> Result<(), SoundcoreError> {
        match &self.rfcomm {
            Some(rfcomm) => {
                rfcomm.send(data).await?;
                Ok(())
            }
            None => Err(SoundcoreError::NotConnected),
        }
    }
    async fn recv(&self) -> Result<Vec<u8>, SoundcoreError> {
        match &self.rfcomm {
            Some(rfcomm) => Ok(remove_padding(rfcomm.recv(1000).await?.as_slice())),
            None => Err(SoundcoreError::BthError {
                source: bluetooth_lib::error::BthError::InvalidSocketError,
            }),
        }
    }

    async fn build_and_send_cmd(
        &self,
        cmd: [i8; 7],
        data: Option<&[u8]>,
    ) -> Result<(), SoundcoreError> {
        todo!()
        // let to_send = build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&cmd), data);
        // let _ = &self.send(&to_send).await?;
        // sleep(SLEEP_DURATION).await;
        // Ok(())
    }

    async fn get_status(&self) -> Result<DeviceStatus, SoundcoreError> {
       todo!()
    }

    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        todo!()
    }
    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError> {
        todo!()
    }

    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError> {
        todo!()
    }
}

impl SoundcoreHearID for A3040 {}
impl SoundcoreANC for A3040 {}
impl SoundcoreLDAC for A3040 {}
impl SoundcoreEQ for A3040 {}