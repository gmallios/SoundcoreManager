/* Draft */

use crate::error;

struct AXXXX {}

impl AXXXX {
    fn new() -> Self {
        Self {}
    }
}

impl SoundcoreBase for AXXXX {}

impl SoundcoreANC for AXXXX {}

trait SoundcoreBase {
    fn get_status(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
    fn get_info(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
    fn get_battery(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
    fn get_fw_version(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
    fn get_sn(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
}

trait SoundcoreANC: SoundcoreBase {
    fn set_anc(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
    fn get_anc(&self) -> Result<(), error::SoundcoreError> {
        Ok(())
    }
}

trait SoundcoreEQ: SoundcoreBase {
    fn set_eq(&self) -> Result<(), error::SoundcoreError>;
    fn get_eq(&self) -> Result<(), error::SoundcoreError>;
}

trait SoundcoreLDAC: SoundcoreBase {
    fn set_ldac(&self) -> Result<(), error::SoundcoreError>;
    fn get_ldac(&self) -> Result<(), error::SoundcoreError>;
}

trait SoundcoreHearID: SoundcoreBase {
    fn set_hearid(&self) -> Result<(), error::SoundcoreError>;
    fn get_hearid(&self) -> Result<(), error::SoundcoreError>;
}
