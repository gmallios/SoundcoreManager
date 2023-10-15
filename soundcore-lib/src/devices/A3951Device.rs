use crate::packets::ResponsePacket;
use crate::{
    base::{SoundcoreANC, SoundcoreDevice, SoundcoreEQ, SoundcoreHearID, SoundcoreLDAC},
    error::SoundcoreError,
    statics::*,
    types::{
        ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave, EQWaveInt,
        ResponseDecoder,
    },
    utils::{
        build_command_array_with_options_toggle_enabled, i8_to_u8vec, remove_padding, verify_resp,
        Clamp,
    },
};
use async_trait::async_trait;
use bluetooth_lib::{platform::RFCOMM, BluetoothAdrr, RFCOMMClient};
use log::debug;
use std::time::Duration;
use tokio::time::sleep;

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(30);

pub static A3951_RFCOMM_UUID: &str = crate::statics::A3951_RFCOMM_UUID;

#[derive(Default)]
pub struct A3951 {
    _btaddr: Option<BluetoothAdrr>,
    rfcomm: Option<RFCOMM>,
}

#[async_trait]
impl SoundcoreDevice for A3951 {
    async fn init(
        &self,
        btaddr: BluetoothAdrr,
    ) -> Result<Box<dyn SoundcoreDevice>, SoundcoreError> {
        let mut rfcomm = RFCOMM::new().await?;
        rfcomm
            .connect_uuid(btaddr.clone(), A3951_RFCOMM_UUID)
            .await?;
        Ok(Box::new(A3951 {
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
        Ok(ResponsePacket::from_bytes(&resp)?.try_into()?)
    }

    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_INFO, None).await?;
        let resp = self.recv().await?;
        if A3951_RESPONSE_VERIFICATION && A3951_RESPONSE_VERIFICATION {
            verify_resp(&resp)?;
        }
        Ok(Self::decode(self, &resp)?)
    }
    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_BATTERYLEVEL, None)
            .await?;
        let resp = self.recv().await?;

        if A3951_RESPONSE_VERIFICATION {
            verify_resp(&resp)?;
        }

        if resp[6] == 4 {
            debug!("Device battery level blink: {:?}", resp);
            // Case battery level. Ignore for now, more debugging needed.
            // Battery charging "blinks" when this event is triggered.
            return Err(SoundcoreError::Unknown);
        }

        Ok(Self::decode(self, &resp[9..11])?)
    }

    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_BATTERYCHARGING, None)
            .await?;
        let resp = self.recv().await?;
        if A3951_RESPONSE_VERIFICATION {
            verify_resp(&resp)?;
        }
        // https://prnt.sc/yze5IvvUtYlq Case battery "blink"
        if resp.len() >= 13 && resp[13] == 255 {
            debug!("Device battery charging blink: {:?}", resp);
            // When "blinking" resp[13] is 255 afaik.
            return Err(SoundcoreError::Unknown);
        }

        Ok(Self::decode(self, &resp[9..11])?)
    }
}

#[async_trait]
impl SoundcoreANC for A3951 {
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
        if A3951_RESPONSE_VERIFICATION {
            verify_resp(&resp)?;
        }
        Ok(ANCProfile::decode(&resp[9..13])?)
    }
}

#[async_trait]
impl SoundcoreEQ for A3951 {
    async fn set_eq(&self, wave: EQWave) -> Result<(), SoundcoreError> {
        let drc_supported = true;
        let eq_index: i32 = 65278; /* Custom EQ Index */
        let eq_hindex = 0; /* I don't know what this is, doesn't seem to change across EQ Indexes and EQ values and is constant */
        let arr_len = match drc_supported {
            /* 76: DRC 74: No DRC */
            true => 76,
            false => 74,
        };
        let drc_offset = match drc_supported {
            true => 4,
            false => 2,
        };
        let mut wave_out: Vec<u8> = vec![0; arr_len];

        wave_out[0] = eq_index as u8;
        wave_out[1] = ((eq_index >> 8) & 0xFF) as u8;

        if drc_supported {
            /* hindex is used on DRC models */
            wave_out[2] = eq_hindex as u8;
            wave_out[3] = ((eq_hindex >> 8) & 0xFF) as u8;
        }

        /* used for both left and right EQs */
        let corrected_eq_wave = EQWave::transform_to_realeq(wave);
        let eq_wave_bytes = EQWaveInt::from_eq_wave(wave).to_bytes();
        let corrected_eq_wave_bytes = EQWaveInt::from_eq_wave(corrected_eq_wave).to_bytes();
        let hearid_wave_bytes = EQWaveInt::from_eq_wave(EQWave::HEARD_ID_DEFAULT).to_bytes();

        /* drc_offset - drc_offset + 16 EQ Wave */
        wave_out[drc_offset..drc_offset + 8].copy_from_slice(&eq_wave_bytes[0..8]);
        wave_out[drc_offset + 8..drc_offset + 16].copy_from_slice(&eq_wave_bytes[0..8]);
        /* Straight from Soundcore spaghetti */
        wave_out[drc_offset + 16] = 255_u8;
        wave_out[drc_offset + 17] = 255_u8;
        wave_out[drc_offset + 18] = 0;
        /* drc_offset + 19-35 HearID EQ Wave */
        wave_out[drc_offset + 19..drc_offset + 27].copy_from_slice(&hearid_wave_bytes[0..8]);
        wave_out[drc_offset + 27..drc_offset + 35].copy_from_slice(&hearid_wave_bytes[0..8]);

        wave_out[drc_offset + 35..drc_offset + 39].copy_from_slice(&[0, 0, 0, 0]);
        wave_out[drc_offset + 39] = 0; /* HearID type */

        /* drc_offset + 40-56 HearID Customer EQ Wave (IDK what this means, hearid data is not reversed atm) */
        wave_out[drc_offset + 40..drc_offset + 48].copy_from_slice(&hearid_wave_bytes[0..8]);
        wave_out[drc_offset + 48..drc_offset + 56].copy_from_slice(&hearid_wave_bytes[0..8]);

        /* drc_offset + 56-72 "Corrected" EQ Wave */
        wave_out[drc_offset + 56..drc_offset + 64].copy_from_slice(&corrected_eq_wave_bytes[0..8]);
        wave_out[drc_offset + 64..drc_offset + 72].copy_from_slice(&corrected_eq_wave_bytes[0..8]);
        self.build_and_send_cmd(A3951_CMD_DEVICE_SETEQ_DRC, Some(&wave_out))
            .await?;
        let _resp = self.recv().await?;
        Ok(())
    }

    async fn get_eq(&self) -> Result<EQWave, SoundcoreError> {
        Ok(self.get_status().await?.left_eq) /* Return both left and right? */
    }
}

#[async_trait]
impl SoundcoreLDAC for A3951 {
    async fn get_ldac(&self) -> Result<bool, SoundcoreError> {
        self.build_and_send_cmd(A3951_CMD_DEVICE_GETLDAC, None)
            .await?;
        let resp = self.recv().await?;
        if A3951_RESPONSE_VERIFICATION {
            verify_resp(&resp)?;
        }
        Ok(resp[9] == 1)
    }

    async fn set_ldac(&self, _enabled: bool) -> Result<(), SoundcoreError> {
        unimplemented!()
    }
}
impl SoundcoreHearID for A3951 {}

impl ResponseDecoder<DeviceInfo> for A3951 {
    fn decode(&self, arr: &[u8]) -> Result<DeviceInfo, SoundcoreError> {
        Ok(DeviceInfo {
            left_fw: String::from_utf8(arr[9..14].to_vec())?,
            right_fw: String::from_utf8(arr[14..19].to_vec())?,
            sn: String::from_utf8(arr[19..35].to_vec())?,
        })
    }
}

impl ResponseDecoder<DeviceStatus> for A3951 {
    fn decode(&self, arr: &[u8]) -> Result<DeviceStatus, SoundcoreError> {
        if arr.len() < 93 {
            return Err(SoundcoreError::RecvError);
        }
        Ok(DeviceStatus {
            host_device: arr[9],
            tws_status: arr[10] == 1,
            battery_level: Self::decode(self, &arr[11..13])?,
            battery_charging: Self::decode(self, &arr[13..15])?,
            left_eq: EQWave::decode(&arr[17..25])?,
            right_eq: EQWave::decode(&arr[25..33])?,
            hearid_enabled: arr[35] == 1,
            left_hearid: EQWave::decode(&arr[36..44])?,
            right_hearid: EQWave::decode(&arr[44..52])?,
            left_hearid_customdata: EQWave::decode(&arr[58..66])?,
            right_hearid_customdata: EQWave::decode(&arr[66..74])?,
            anc_status: ANCProfile::decode(&arr[86..90])?,
            side_tone_enabled: arr[90] == 1,
            wear_detection_enabled: arr[91] == 1,
            touch_tone_enabled: arr[92] == 1,
        })
    }
}

impl ResponseDecoder<BatteryLevel> for A3951 {
    fn decode(&self, arr: &[u8]) -> Result<BatteryLevel, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::InvalidResponseLength {
                expected: 2,
                got: arr.len(),
                data: arr.to_vec(),
            });
        }

        Ok(BatteryLevel {
            left: Clamp::clamp(arr[0], 0, 5),
            right: Clamp::clamp(arr[1], 0, 5),
        })
    }
}

impl ResponseDecoder<BatteryCharging> for A3951 {
    fn decode(&self, arr: &[u8]) -> Result<BatteryCharging, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::InvalidResponseLength {
                expected: 2,
                got: arr.len(),
                data: arr.to_vec(),
            });
        }

        Ok(BatteryCharging {
            left: arr[0] == 1,
            right: arr[1] == 1,
        })
    }
}

impl ANCProfile {
    pub const NORMAL_MODE: ANCProfile = ANCProfile {
        option: 2,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const ANC_TRANSPORT_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_OUTDOOR_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 1,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_INDOOR_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 2,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_FULLY_TRANSPARENT_MODE: ANCProfile = ANCProfile {
        option: 1,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_VOCAL_MODE: ANCProfile = ANCProfile {
        option: 1,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub fn anc_custom_value(val: u8) -> ANCProfile {
        ANCProfile {
            option: 0,
            anc_option: 3,
            transparency_option: 1,
            anc_custom: Clamp::clamp(val, 0, 10),
        }
    }

    pub fn decode(arr: &[u8]) -> Result<ANCProfile, std::string::FromUtf8Error> {
        let anc_custom: u8;

        if arr[3] == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(arr[3], 0, 10);
        }

        Ok(ANCProfile {
            option: Clamp::clamp(arr[0], 0, 2),
            anc_option: Clamp::clamp(arr[1], 0, 3),
            transparency_option: arr[2],
            anc_custom,
        })
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let anc_custom: u8;

        if self.anc_custom == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(self.anc_custom, 0, 10);
        }

        [
            Clamp::clamp(self.option, 0, 2),
            Clamp::clamp(self.anc_option, 0, 3),
            self.transparency_option,
            anc_custom,
        ]
    }
}

#[cfg(test)]
impl A3951 {
    /* Used for comparing in testing */
    pub fn device_status(bytes: &[u8]) -> DeviceStatus {
        Self::decode(
            &A3951 {
                _btaddr: None,
                rfcomm: None,
            },
            bytes,
        )
        .unwrap()
    }
}
