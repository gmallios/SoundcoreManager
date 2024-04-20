use crate::{
    devices::{A3040SoundModeUpdateCommand, A3951SoundModeUpdateCommand},
    models::{ANCMode, SoundMode, TransparencyMode},
    packets::Packet,
    types::SupportedModels,
};

pub struct SoundModeCommandBuilder {
    sound_mode: SoundMode,
    model: SupportedModels,
}

impl SoundModeCommandBuilder {
    pub fn new(sound_mode: SoundMode, model: SupportedModels) -> Self {
        Self { sound_mode, model }
    }

    pub fn build(self) -> Vec<u8> {
        match self.model {
            SupportedModels::A3040 => A3040SoundModeUpdateCommand::new(self.sound_mode).bytes(),
            SupportedModels::A3951 => A3951SoundModeUpdateCommand::new(self.sound_mode).bytes(),
            _ => self.find_builder(),
        }
    }

    fn find_builder(&self) -> Vec<u8> {
        if let (ANCMode::SceneBased(_), TransparencyMode::NonCustomizable(_), None) = (
            self.sound_mode.anc_mode,
            self.sound_mode.trans_mode,
            self.sound_mode.custom_trans,
        ) {
            A3951SoundModeUpdateCommand::new(self.sound_mode).bytes()
        } else {
            A3040SoundModeUpdateCommand::new(self.sound_mode).bytes()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ANCMode, CurrentSoundMode, CustomANCValue, SoundMode, TransparencyMode};

    #[test]
    fn test_find_builder() {
        let sound_mode = SoundMode {
            current: CurrentSoundMode::Normal,
            anc_mode: ANCMode::SceneBased(crate::models::SceneBasedANCMode::Outdoor),
            trans_mode: TransparencyMode::NonCustomizable(
                crate::models::NonCustomizableTransparencyMode::FullyTransparent,
            ),
            custom_anc: CustomANCValue::from_u8(0),
            custom_trans: None,
        };
        let builder = SoundModeCommandBuilder::new(sound_mode, SupportedModels::A3027);
        let bytes = builder.build();
        assert_eq!(bytes, [8, 238, 0, 0, 0, 6, 129, 14, 0, 2, 1, 0, 0, 142]);
    }
}
