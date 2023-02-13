use async_trait::async_trait;
use bluetooth_lib::{platform::RFCOMM, BluetoothAdrr, RFCOMMClient};
use log::debug;
use tokio::time::sleep;

use crate::{
    base::{SoundcoreANC, SoundcoreDevice, SoundcoreEQ, SoundcoreHearID, SoundcoreLDAC},
    error::SoundcoreError,
    statics::*,
    types::{
        ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave, EQWaveInt, ResponseDecoder
    },
    utils::{build_command_array_with_options_toggle_enabled, i8_to_u8vec, verify_resp, Clamp},
};
use std::time::Duration;

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(30);


pub struct A3027 {
    btaddr: Option<BluetoothAdrr>,
    rfcomm: Option<RFCOMM>,
}

impl Default for A3027 {
    fn default() -> Self {
        Self {
            btaddr: None,
            rfcomm: None,
        }
    }
}

#[async_trait]
impl SoundcoreDevice for A3027 {
    async fn init(&self, btaddr: BluetoothAdrr) -> Result<Box<dyn SoundcoreDevice>, SoundcoreError> {
        let mut rfcomm = RFCOMM::new().await?;
        rfcomm
            .connect_uuid(btaddr.clone(), A3951_RFCOMM_UUID)
            .await?;
        Ok(Box::new(A3027 { btaddr: Some(btaddr), rfcomm: Some(rfcomm) }))
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
    async fn recv(&self, size: usize) -> Result<Vec<u8>, SoundcoreError> {
        match &self.rfcomm {
            Some(rfcomm) => Ok(rfcomm.recv(size).await?),
            None => Err(SoundcoreError::NotConnected),
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
        self.build_and_send_cmd(A3951_CMD_DEVICE_STATUS, None).await?;
        let resp = self.recv(97).await?;
        // if !verify_resp(&resp) {
        //     return Err(SoundcoreError::ResponseChecksumError);
        // }
        Ok(Self::decode(&resp)?)
    }

    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_INFO, None).await?;
        let resp = self.recv(36).await?;
        // if !verify_resp(&resp) {
        //     return Err(SoundcoreError::ResponseChecksumError);
        // }
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
impl SoundcoreANC for A3027 {
    async fn set_anc(&self, profile: ANCProfile) -> Result<(), crate::error::SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_SETANC, Some(&profile.to_bytes()))
            .await?;
        let _resp = self.recv(10).await?; /* No response validation - Need more info */
        Ok(())
    }

    async fn get_anc(&self) -> Result<ANCProfile, crate::error::SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_GETANC, None)
            .await?;
        let resp = self.recv(14).await?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(ANCProfile::decode(&resp[9..13])?)
    }
}

#[async_trait]
impl SoundcoreEQ for A3027 {
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
        let _resp = self.recv(100).await?;
        Ok(())
    }

    async fn get_eq(&self) -> Result<EQWave, SoundcoreError> {
        Ok(self.get_status().await?.left_eq) /* Return both left and right? */
    }

}

#[async_trait]
impl SoundcoreLDAC for A3027 {
    async fn get_ldac(&self) -> Result<bool, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_GETLDAC, None).await?;
        let resp = self.recv(11).await?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(resp[9] == 1)
    }

    async fn set_ldac(&self, enabled: bool) -> Result<(), SoundcoreError> {
        unimplemented!()
    }
}
impl SoundcoreHearID for A3027 {}

impl ResponseDecoder<DeviceInfo> for A3027 {
    fn decode(arr: &[u8]) -> Result<DeviceInfo, SoundcoreError> {
        Ok(DeviceInfo {
            left_fw: String::from_utf8(arr[9..14].to_vec())?,
            right_fw: String::from_utf8(arr[14..19].to_vec())?,
            sn: String::from_utf8(arr[19..35].to_vec())?,
        })
    }
}

impl ResponseDecoder<DeviceStatus> for A3027 {
    fn decode(arr: &[u8]) -> Result<DeviceStatus, SoundcoreError> {
        if arr.len() < 93 {
            return Err(SoundcoreError::Unknown);
        }
        let chargeArr = vec![arr[10], arr[10]];
        let levelArr = vec![arr[9], arr[9]];

        Ok(DeviceStatus {
            host_device: arr[9],
            tws_status: arr[10] == 1,
            battery_level: Self::decode(&*levelArr)?,
            battery_charging: Self::decode(&*chargeArr)?,
            left_eq: EQWave::decode(&arr[13..21])?,
            right_eq: EQWave::decode(&arr[13..21])?,
            hearid_enabled: arr[23] == 1,
            left_hearid: EQWave::decode(&arr[24..32])?,
            right_hearid: EQWave::decode(&arr[32..40])?,
            left_hearid_customdata: EQWave::default(),
            right_hearid_customdata: EQWave::default(),
            anc_status: ANCProfile::decode(&arr[44..48])?,
            side_tone_enabled: false,
            wear_detection_enabled: arr[69] == 1,
            touch_tone_enabled: false,
        })
    }
}


impl ResponseDecoder<BatteryLevel> for A3027 {
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

impl ResponseDecoder<BatteryCharging> for A3027 {
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