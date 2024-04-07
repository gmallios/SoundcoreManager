use crate::{models::SoundMode, packets::Packet};

pub struct A3040SoundModeUpdateCommand {
    sound_mode: SoundMode,
}

impl A3040SoundModeUpdateCommand {
    pub fn new(sound_mode: SoundMode) -> Self {
        Self { sound_mode }
    }
}

impl Packet for A3040SoundModeUpdateCommand {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x06, 0x81]
    }

    fn payload(&self) -> Vec<u8> {
        self.sound_mode.to_bytes_with_custom_transparency().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::models::{
        ANCMode, AdaptiveANCMode, CurrentSoundMode, CustomANCValue, CustomTransparencyValue,
        CustomizableTransparencyMode, TransparencyMode,
    };

    #[test]
    fn test_sound_mode_update_command() {
        let command = A3040SoundModeUpdateCommand {
            sound_mode: SoundMode {
                current: CurrentSoundMode::ANC,
                anc_mode: ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
                custom_anc: CustomANCValue::from_u8(5),
                trans_mode: TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
                custom_trans: Some(CustomTransparencyValue::from_u8(3)),
            },
        };

        assert_eq!(
            command.bytes(),
            vec![
                0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x10, 0x00, 0x00, 0x51, 0x01, 0x01, 0x00,
                0x03, 0xe3,
            ]
        );
    }
}
