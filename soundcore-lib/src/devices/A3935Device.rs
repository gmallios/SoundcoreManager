use async_trait::async_trait;
use bluetooth_lib::{platform::RFCOMM, BluetoothAdrr, RFCOMMClient};
use log::debug;
use tokio::time::sleep;

use crate::{
    base::{SoundcoreANC, SoundcoreDevice, SoundcoreEQ, SoundcoreHearID, SoundcoreLDAC},
    error::SoundcoreError,
    statics::*,
    types::{
        ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave, EQWaveInt,
        ResponseDecoder,
    },
    utils::{build_command_array_with_options_toggle_enabled, i8_to_u8vec, verify_resp, Clamp, remove_padding},
};
use std::time::Duration;

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(30);

#[derive(Default)]
pub struct A3935 {
    btaddr: Option<BluetoothAdrr>,
    rfcomm: Option<RFCOMM>,
}

#[async_trait]
impl SoundcoreDevice for A3935 {
    async fn init(
        &self,
        btaddr: BluetoothAdrr,
    ) -> Result<Box<dyn SoundcoreDevice>, SoundcoreError> {
        let mut rfcomm = RFCOMM::new().await?;
        rfcomm
            .connect_uuid(btaddr.clone(), A3951_RFCOMM_UUID)
            .await?;
        Ok(Box::new(A3935 {
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
            Some(rfcomm) => Ok(remove_padding(rfcomm.recv(300).await?.as_slice())),
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
        let to_send = build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&cmd), data);
        let _ = &self.send(&to_send).await?;
        sleep(SLEEP_DURATION).await;
        Ok(())
    }

    async fn get_status(&self) -> Result<DeviceStatus, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_STATUS, None)
            .await?;
        let resp = self.recv().await?;
        verify_resp(&resp)?;
        Ok(Self::decode(&resp)?)
    }

    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_INFO, None).await?;
        let resp = self.recv().await?;
        verify_resp(&resp)?;
        Ok(Self::decode(&resp)?)
    }
    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError> {
        Ok(self.get_status().await?.battery_level)
    }

    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError> {
        Ok(self.get_status().await?.battery_charging)
    }
}

#[async_trait]
impl SoundcoreANC for A3935 {
    async fn set_anc(&self, profile: ANCProfile) -> Result<(), crate::error::SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_SETANC, Some(&profile.to_bytes()))
            .await?;
        let _resp = self.recv().await?; /* No response validation - Need more info */
        Ok(())
    }

    async fn get_anc(&self) -> Result<ANCProfile, crate::error::SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_GETANC, None)
            .await?;
        let resp = self.recv().await?;
        verify_resp(&resp)?;
        Ok(ANCProfile::decode(&resp[9..13])?)
    }
}

#[async_trait]
impl SoundcoreEQ for A3935 {
    async fn set_eq(&self, wave: EQWave) -> Result<(), SoundcoreError> {
        /* Original Java method name: SendEQ_NoDrc_Not_A3951_A3930 */
        let mut wave_out = vec![0; 10];
        let eq_index: i32 = 65278; /* Custom EQ Index */
        let eq_wave = EQWaveInt::from_eq_wave(wave).to_8bytes();
        wave_out[0] = eq_index as u8;
        wave_out[1] = (eq_index >> 8) as u8;
        wave_out[2..10].copy_from_slice(&eq_wave);

        /* A3027 Doesn't appear to be using DRC */
        self.build_and_send_cmd(A3027_CMD_DEVICE_SETEQ, Some(&wave_out))
            .await?;
        let _resp = self.recv().await?;
        Ok(())
    }

    async fn get_eq(&self) -> Result<EQWave, SoundcoreError> {
        Ok(self.get_status().await?.left_eq) /* Return both left and right? */
    }
}

#[async_trait]
impl SoundcoreLDAC for A3935 {
    async fn get_ldac(&self) -> Result<bool, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_GETLDAC, None)
            .await?;
        let resp = self.recv().await?;
        verify_resp(&resp)?;
        Ok(resp[9] == 1)
    }

    async fn set_ldac(&self, enabled: bool) -> Result<(), SoundcoreError> {
        unimplemented!()
    }
}
impl SoundcoreHearID for A3935 {}

impl ResponseDecoder<DeviceInfo> for A3935 {
    fn decode(arr: &[u8]) -> Result<DeviceInfo, SoundcoreError> {
        Ok(DeviceInfo {
            left_fw: String::from_utf8(arr[9..14].to_vec())?,
            right_fw: String::from_utf8(arr[14..19].to_vec())?,
            sn: String::from_utf8(arr[19..35].to_vec())?,
        })
    }
}

impl ResponseDecoder<DeviceStatus> for A3935 {
    fn decode(arr: &[u8]) -> Result<DeviceStatus, SoundcoreError> {
        if arr.len() < 93 {
            return Err(SoundcoreError::RecvError);
        }

        Ok(DeviceStatus {
            host_device: arr[9],
            tws_status: arr[10] == 1,
            battery_level: Self::decode(&arr[11..13])?,
            battery_charging: Self::decode(&arr[13..15])?,
            left_eq: EQWave::decode(&arr[17..25])?,
            right_eq: EQWave::decode(&arr[25..33])?,
            hearid_enabled: false,
            left_hearid: EQWave::default(), /* A3935 Doesn't Seem to support hearID */
            right_hearid: EQWave::default(),
            left_hearid_customdata: EQWave::default(),
            right_hearid_customdata: EQWave::default(),
            anc_status: ANCProfile::decode(&arr[45..49])?,
            side_tone_enabled: arr[49] == 1,
            touch_tone_enabled: arr[50] == 1,
            wear_detection_enabled: false, /* Doesn't seem to support it? */
                                           // TODO: This device supports AutoPowerOff
                                           // TODO: arr[51] == 1, enable auto power off
                                           // TODO: arr[52] is the index of the auto power off time
        })
    }
}

impl ResponseDecoder<BatteryLevel> for A3935 {
    fn decode(arr: &[u8]) -> Result<BatteryLevel, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryLevel {
            left: Clamp::clamp(arr[0], 0, 5),
            right: Clamp::clamp(arr[1], 0, 5),
        })
    }
}

impl ResponseDecoder<BatteryCharging> for A3935 {
    fn decode(arr: &[u8]) -> Result<BatteryCharging, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryCharging {
            left: arr[0] == 1,
            right: arr[1] == 1,
        })
    }
}
