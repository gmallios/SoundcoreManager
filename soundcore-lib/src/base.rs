use async_trait::async_trait;
use bluetooth_lib::BluetoothAdrr;

use crate::{
    error::SoundcoreError,
    types::{
        ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave,
        ResponseDecoder,
    },
};

/* Should we require all traits or make this be a SoundcoreBase and have all
other traits be "extensions" (may require unsafe downcasting/upcasting) */
/* We can't require Default trait as it implements static methods and makes SoundcoreDevice not object safe. */

#[async_trait]
pub trait SoundcoreDevice:
    SoundcoreANC
    + SoundcoreEQ
    + SoundcoreLDAC
    + SoundcoreHearID
    + ResponseDecoder<DeviceInfo>
    + ResponseDecoder<DeviceStatus>
    + Send
    + Sync
{
    async fn init(&self, btaddr: BluetoothAdrr)
        -> Result<Box<dyn SoundcoreDevice>, SoundcoreError>;
    async fn close(&self) -> Result<(), SoundcoreError>;
    async fn send(&self, data: &[u8]) -> Result<(), SoundcoreError>;
    async fn recv(&self) -> Result<Vec<u8>, SoundcoreError>;
    async fn build_and_send_cmd(
        &self,
        cmd: [i8; 7],
        data: Option<&[u8]>,
    ) -> Result<(), SoundcoreError>;
    async fn get_status(&self) -> Result<DeviceStatus, SoundcoreError>;
    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreError>;
    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError>;
    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError>;
}

/* "Optional" traits - Not really since SoundcoreDevice
has a bound on them but they return Err by default */
#[async_trait]
pub trait SoundcoreANC: Sync + Send {
    async fn set_anc(&self, _profile: ANCProfile) -> Result<(), SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "ANC - set_anc".to_string(),
        ))
    }
    async fn get_anc(&self) -> Result<ANCProfile, SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "ANC - get_anc".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreEQ: Sync + Send {
    async fn set_eq(&self, _wave: EQWave) -> Result<(), SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "EQ - set_eq".to_string(),
        ))
    }
    async fn get_eq(&self) -> Result<EQWave, SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "EQ - get_eq".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreLDAC: Sync + Send {
    async fn set_ldac(&self, _toggle: bool) -> Result<(), SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "LDAC - set_ldac".to_string(),
        ))
    }
    async fn get_ldac(&self) -> Result<bool, SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "LDAC - get_ldac".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreHearID: Sync + Send {
    async fn set_hearid(&self) -> Result<(), SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "HearID - set_hearid".to_string(),
        ))
    }
    async fn get_hearid(&self) -> Result<(), SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "HearID - get_hearid".to_string(),
        ))
    }
}
