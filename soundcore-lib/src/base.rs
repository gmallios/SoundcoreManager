use async_trait::async_trait;
use bluetooth_lib::BluetoothAdrr;

use crate::{
    error::SoundcoreLibError,
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
    async fn init(
        &self,
        btaddr: BluetoothAdrr,
    ) -> Result<Box<dyn SoundcoreDevice>, SoundcoreLibError>;
    async fn close(&self) -> Result<(), SoundcoreLibError>;
    async fn send(&self, data: &[u8]) -> Result<(), SoundcoreLibError>;
    async fn recv(&self) -> Result<Vec<u8>, SoundcoreLibError>;
    async fn build_and_send_cmd(
        &self,
        cmd: [i8; 7],
        data: Option<&[u8]>,
    ) -> Result<(), SoundcoreLibError>;
    async fn get_status(&self) -> Result<DeviceStatus, SoundcoreLibError>;
    async fn get_info(&self) -> Result<DeviceInfo, SoundcoreLibError>;
    async fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreLibError>;
    async fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreLibError>;
}

/* "Optional" traits - Not really since SoundcoreDevice
has a bound on them but they return Err by default */
#[async_trait]
pub trait SoundcoreANC: Sync + Send {
    async fn set_anc(&self, _profile: ANCProfile) -> Result<(), SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "ANC - set_anc".to_string(),
        ))
    }
    async fn get_anc(&self) -> Result<ANCProfile, SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "ANC - get_anc".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreEQ: Sync + Send {
    async fn set_eq(&self, _wave: EQWave) -> Result<(), SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "EQ - set_eq".to_string(),
        ))
    }
    async fn get_eq(&self) -> Result<EQWave, SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "EQ - get_eq".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreLDAC: Sync + Send {
    async fn set_ldac(&self, _toggle: bool) -> Result<(), SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "LDAC - set_ldac".to_string(),
        ))
    }
    async fn get_ldac(&self) -> Result<bool, SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "LDAC - get_ldac".to_string(),
        ))
    }
}
#[async_trait]
pub trait SoundcoreHearID: Sync + Send {
    async fn set_hearid(&self) -> Result<(), SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "HearID - set_hearid".to_string(),
        ))
    }
    async fn get_hearid(&self) -> Result<(), SoundcoreLibError> {
        Err(SoundcoreLibError::FeatureNotSupported(
            "HearID - get_hearid".to_string(),
        ))
    }
}
