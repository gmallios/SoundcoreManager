use std::{slice::from_ref, time::Duration};

use crate::{
    base::{SoundcoreANC, SoundcoreDevice, SoundcoreEQ, SoundcoreHearID, SoundcoreLDAC},
    error::SoundcoreError,
    statics::{
        A3040_CMD_DEVICE_BATTERYLEVEL, A3040_CMD_DEVICE_CHARGINSTATUS, A3040_CMD_DEVICE_INFO,
        A3040_RFCOMM_UUID,
    },
    types::{
        ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave,
        ResponseDecoder,
    },
    utils::{build_command_with_options, i8_to_u8vec, remove_padding, Clamp},
};
use bluetooth_lib::{platform::RFCOMM, BluetoothAdrr, RFCOMMClient};
use tokio::time::sleep;

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(30);

#[derive(Default)]
pub struct A3040 {
    _btaddr: Option<BluetoothAdrr>,
    rfcomm: Option<RFCOMM>,
}

#[async_trait::async_trait]
impl SoundcoreDevice for A3040 {
    async fn init(
        &self,
        btaddr: BluetoothAdrr,
    ) -> Result<Box<dyn SoundcoreDevice>, SoundcoreError> {
        let mut rfcomm = RFCOMM::new().await?;
        rfcomm
            .connect_uuid(btaddr.clone(), A3040_RFCOMM_UUID)
            .await?;
        Ok(Box::new(A3040 {
            _btaddr: Some(btaddr),
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
        let to_send = build_command_with_options(&i8_to_u8vec(&cmd), data);
        let _ = &self.send(&to_send).await?;
        sleep(SLEEP_DURATION).await;
        Ok(())
    }

    /* DeviceInfo and Status is the same command and response */
    async fn get_status(&self) -> Result<DeviceStatus, SoundcoreError> {
        self.build_and_send_cmd(A3040_CMD_DEVICE_INFO, None).await?;
        let resp = self.recv().await?;
        Ok(Self::decode(self, &resp)?)
    }

    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        self.build_and_send_cmd(A3040_CMD_DEVICE_INFO, None).await?;
        let resp = self.recv().await?;
        Ok(Self::decode(self, &resp)?)
    }

    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError> {
        self.build_and_send_cmd(A3040_CMD_DEVICE_BATTERYLEVEL, None)
            .await?;
        let resp = self.recv().await?;
        Ok(Self::decode(self, &resp)?)
    }

    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError> {
        self.build_and_send_cmd(A3040_CMD_DEVICE_CHARGINSTATUS, None)
            .await?;
        let resp = self.recv().await?;
        Ok(Self::decode(self, &resp)?)
    }
}

impl ResponseDecoder<DeviceInfo> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<DeviceInfo, SoundcoreError> {
        if arr.len() < 85 {
            return Err(SoundcoreError::InvalidResponse);
        }

        Ok(DeviceInfo {
            left_fw: String::from_utf8(arr[11..16].to_vec())?,
            right_fw: String::from_utf8(arr[11..16].to_vec())?,
            sn: String::from_utf8(arr[16..32].to_vec())?,
        })
    }
}
impl ResponseDecoder<DeviceStatus> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<DeviceStatus, SoundcoreError> {
        if arr.len() < 85 {
            return Err(SoundcoreError::InvalidResponse);
        }

        let base = arr[54] as usize;
        let anc_current_mode_idx = (base + 54) - 1;
        let anc_model_idx = anc_current_mode_idx + 1;
        let touch_tone_idx = anc_model_idx + 6;
        let wear_detection_idx = touch_tone_idx + 1;
        /* The rest of the indexes are on JADX->A3040AnalysisService */

        Ok(DeviceStatus {
            host_device: 0, // Not available
            tws_status: true,
            battery_level: Self::decode(self, from_ref(&arr[9]))?,
            battery_charging: Self::decode(self, from_ref(&arr[10]))?,
            anc_status: Self::decode(self, from_ref(&arr[anc_current_mode_idx]))?,
            side_tone_enabled: false,
            wear_detection_enabled: arr[wear_detection_idx] == 1,
            touch_tone_enabled: arr[touch_tone_idx] == 1,
            left_eq: Self::decode(self, &arr[34..44])?,
            right_eq: Self::decode(self, &arr[34..44])?,
            hearid_enabled: false, // Doesn't seem to be supported
            left_hearid: EQWave::default(),
            right_hearid: EQWave::default(),
            left_hearid_customdata: EQWave::default(),
            right_hearid_customdata: EQWave::default(),
        })
    }
}

impl ResponseDecoder<BatteryLevel> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<BatteryLevel, SoundcoreError> {
        Ok(BatteryLevel {
            left: Clamp::clamp(arr[9], 0, 5),
            right: Clamp::clamp(arr[10], 0, 5),
        })
    }
}

impl ResponseDecoder<BatteryCharging> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<BatteryCharging, SoundcoreError> {
        Ok(BatteryCharging {
            left: arr[9] == 1,
            right: arr[10] == 1,
        })
    }
}

impl ResponseDecoder<ANCProfile> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<ANCProfile, SoundcoreError> {
        match arr.first() {
            Some(&byte) if get_nth_bit_value(byte, 1) == 1 => Ok(ANCProfile::ANC_OUTDOOR_MODE),
            Some(&byte) if get_nth_bit_value(byte, 2) == 1 => {
                Ok(ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE)
            }
            Some(&byte) if get_nth_bit_value(byte, 3) == 1 => Ok(ANCProfile::NORMAL_MODE),
            _ => Err(SoundcoreError::InvalidResponse),
        }
    }
}

impl ResponseDecoder<EQWave> for A3040 {
    fn decode(&self, arr: &[u8]) -> Result<EQWave, SoundcoreError> {
        Ok(EQWave {
            pos0: Clamp::clamp(arr[0], 60, 180) as f32 / 10.0,
            pos1: Clamp::clamp(arr[1], 60, 180) as f32 / 10.0,
            pos2: Clamp::clamp(arr[2], 60, 180) as f32 / 10.0,
            pos3: Clamp::clamp(arr[3], 60, 180) as f32 / 10.0,
            pos4: Clamp::clamp(arr[4], 60, 180) as f32 / 10.0,
            pos5: Clamp::clamp(arr[5], 60, 180) as f32 / 10.0,
            pos6: Clamp::clamp(arr[6], 60, 180) as f32 / 10.0,
            pos7: Clamp::clamp(arr[7], 60, 180) as f32 / 10.0,
            pos8: Clamp::clamp(arr[8], 60, 180) as f32 / 10.0,
            pos9: Clamp::clamp(arr[9], 60, 180) as f32 / 10.0,
        })
    }
}

impl SoundcoreHearID for A3040 {}
impl SoundcoreANC for A3040 {}
impl SoundcoreLDAC for A3040 {}
impl SoundcoreEQ for A3040 {}

fn get_nth_bit_value(b: u8, n: u8) -> u8 {
    // shift the byte n-1 bits to the right and bitwise AND it with 1 to get the nth bit value
    (b >> (n - 1)) & 1
}
