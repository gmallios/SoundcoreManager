/* Draft */

use std::any::Any;

use crate::{
    error::{self, SoundcoreError},
    types::{RecvFnType, SendFnType},
};

/* Should we require all traits or make this be a SoundcoreBase and have all
other traits be "extensions" (may require unsafe downcasting/upcasting) */
trait SoundcoreDevice:
    SoundcoreANC + SoundcoreEQ + SoundcoreLDAC + SoundcoreHearID + Send + Sync
{
    fn new(send_cb: SendFnType, recv_cb: RecvFnType) -> Self;
    fn send(&self, data: &[u8]) -> Result<(), error::SoundcoreError>;
    fn recv(&self, size: usize) -> Result<Vec<u8>, error::SoundcoreError>;
    fn get_status(&self) -> Result<(), error::SoundcoreError>;
    fn get_info(&self) -> Result<(), error::SoundcoreError>;
    fn get_battery(&self) -> Result<(), error::SoundcoreError>;
    fn get_fw_version(&self) -> Result<(), error::SoundcoreError>;
    fn get_sn(&self) -> Result<(), error::SoundcoreError>;
}

/* "Optional" traits - Not really since SoundcoreDevice
has a bound on them but they return Err by default */
trait SoundcoreANC {
    fn set_anc(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "ANC - set_ancc".to_string(),
        ))
    }
    fn get_anc(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "ANC - get_anc".to_string(),
        ))
    }
}

trait SoundcoreEQ {
    fn set_eq(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "EQ - set_eq".to_string(),
        ))
    }
    fn get_eq(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "EQ - get_eq".to_string(),
        ))
    }
}

trait SoundcoreLDAC {
    fn set_ldac(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "LDAC - set_ldac".to_string(),
        ))
    }
    fn get_ldac(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "LDAC - get_ldac".to_string(),
        ))
    }
}

trait SoundcoreHearID {
    fn set_hearid(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "HearID - set_hearid".to_string(),
        ))
    }
    fn get_hearid(&self) -> Result<(), error::SoundcoreError> {
        Err(SoundcoreError::FeatureNotSupported(
            "HearID - get_hearid".to_string(),
        ))
    }
}
