use crate::{
    models::{CustomHearID, EQConfiguration, StereoEQConfiguration},
    packets::Packet,
};

pub struct A3951EqUpdateCommand {
    eq_configuration: StereoEQConfiguration,
    hear_id: Option<CustomHearID>,
}

impl A3951EqUpdateCommand {
    const DEFAULT_HEAR_ID_EQ: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    pub fn new(eq_configuration: EQConfiguration) -> Self {
        Self {
            eq_configuration: eq_configuration.into(),
            hear_id: None,
        }
    }
}

impl Packet for A3951EqUpdateCommand {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x03, 0x87]
    }

    fn payload(&self) -> Vec<u8> {
        // 2 bytes profile - FEFE - Custom
        let profile_bytes = [
            self.eq_configuration.profile.id() as u8,
            (self.eq_configuration.profile.id() >> 8) as u8,
        ];
        let hear_id = self.hear_id.clone().unwrap_or_default();
        let hear_id_eq_idx_bytes = hear_id.hear_id_eq_index.unwrap_or(0).to_be_bytes();
        let no_drc_eq_bytes_left = self.eq_configuration.eq.left.to_8band_bytes();
        let no_drc_eq_bytes_right = self.eq_configuration.eq.right.to_8band_bytes();
        // TODO: Refactor HearID and parsers to include these
        let hear_id_gender = 0xFF;
        let hear_id_age_range = 0xFF;

        let (hear_id_eq_left, hear_id_eq_right) = match &self.hear_id {
            Some(hear_id) => (
                hear_id.clone().custom_values.left.to_8band_bytes(),
                hear_id.clone().custom_values.right.to_8band_bytes(),
            ),
            None => (
                Self::DEFAULT_HEAR_ID_EQ.to_vec(),
                Self::DEFAULT_HEAR_ID_EQ.to_vec(),
            ),
        };

        let hear_id_time: [u8; 4] = hear_id.base.time.to_be_bytes();
        let hear_id_type = hear_id.hearid_type.0;

        let (hear_id_custom_left, hear_id_custom_right) = match &self.hear_id {
            Some(hearid) => (
                hearid.custom_values.left.to_8band_bytes(),
                hearid.custom_values.right.to_8band_bytes(),
            ),
            None => (
                Self::DEFAULT_HEAR_ID_EQ.to_vec(),
                Self::DEFAULT_HEAR_ID_EQ.to_vec(),
            ),
        };

        let mut drc_eq_bytes_left = self.eq_configuration.eq.left.to_drc_bytes();
        let mut drc_eq_bytes_right = self.eq_configuration.eq.right.to_drc_bytes();
        drc_eq_bytes_left.truncate(8);
        drc_eq_bytes_right.truncate(8);

        let mut bytes = Vec::with_capacity(96);
        bytes.extend_from_slice(&profile_bytes);
        bytes.extend_from_slice(&hear_id_eq_idx_bytes);
        bytes.extend_from_slice(&no_drc_eq_bytes_left);
        bytes.extend_from_slice(&no_drc_eq_bytes_right);
        bytes.push(hear_id_gender);
        bytes.push(hear_id_age_range);
        bytes.push(0x00);
        bytes.extend_from_slice(&hear_id_eq_left);
        bytes.extend_from_slice(&hear_id_eq_right);
        bytes.extend_from_slice(&hear_id_time);
        bytes.push(hear_id_type);
        bytes.extend_from_slice(&hear_id_custom_left);
        bytes.extend_from_slice(&hear_id_custom_right);
        bytes.extend_from_slice(&drc_eq_bytes_left);
        bytes.extend_from_slice(&drc_eq_bytes_right);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::models::EQProfile;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deep_eq_update_command() {
        let eq_configuration = EQConfiguration::stereo_with_profile(EQProfile::Deep);
        let command = A3951EqUpdateCommand::new(eq_configuration);
        assert_eq!(
            test_data::a3951::A3951_EQ_UPDATE_DEEP_NO_HEAR_ID.to_vec(),
            command.bytes()
        );
    }
}
