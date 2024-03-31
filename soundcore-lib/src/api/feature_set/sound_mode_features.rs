use std::sync::Arc;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::models::{
    ANCMode, AdaptiveANCMode, CustomizableTransparencyMode, NonCustomizableTransparencyMode,
    SceneBasedANCMode, TransparencyMode,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typeshare]
#[serde(rename_all = "camelCase")]
pub struct SoundModeFeatures {
    allowed_anc_modes: Arc<[ANCMode]>,
    allowed_transparency_modes: Arc<[TransparencyMode]>,
    has_normal_mode: bool,
}

impl SoundModeFeatures {
    const ANC_MODES_WITH_CUSTOM_VALUE: [ANCMode; 2] = [
        ANCMode::Adaptive(AdaptiveANCMode::Custom),
        ANCMode::SceneBased(SceneBasedANCMode::Custom),
    ];

    const TRANS_MODES_WITH_CUSTOM_VALUE: [TransparencyMode; 1] = [TransparencyMode::Customizable(
        CustomizableTransparencyMode::Custom,
    )];

    pub fn new(
        anc_modes: &[ANCMode],
        transparency_modes: &[TransparencyMode],
        normal_mode: bool,
    ) -> Self {
        Self {
            allowed_anc_modes: anc_modes.into(),
            allowed_transparency_modes: transparency_modes.into(),
            has_normal_mode: normal_mode,
        }
    }

    pub fn adaptive_customizable_anc_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures {
            allowed_anc_modes: [
                ANCMode::Adaptive(AdaptiveANCMode::Custom),
                ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
            ]
            .into(),
            allowed_transparency_modes: [
                TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
                TransparencyMode::Customizable(CustomizableTransparencyMode::TalkMode),
            ]
            .into(),
            has_normal_mode: true,
        }
    }

    pub fn scene_based_customizable_anc_non_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures {
            allowed_anc_modes: [
                ANCMode::SceneBased(SceneBasedANCMode::Custom),
                ANCMode::SceneBased(SceneBasedANCMode::Indoor),
                ANCMode::SceneBased(SceneBasedANCMode::Outdoor),
                ANCMode::SceneBased(SceneBasedANCMode::Transport),
            ]
            .into(),
            allowed_transparency_modes: [
                TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::FullyTransparent,
                ),
                TransparencyMode::NonCustomizable(NonCustomizableTransparencyMode::Vocal),
            ]
            .into(),
            has_normal_mode: true,
        }
    }

    pub fn scene_based_non_customizable_anc_non_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures {
            allowed_anc_modes: [
                ANCMode::SceneBased(SceneBasedANCMode::Indoor),
                ANCMode::SceneBased(SceneBasedANCMode::Outdoor),
                ANCMode::SceneBased(SceneBasedANCMode::Transport),
            ]
            .into(),
            allowed_transparency_modes: [
                TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::FullyTransparent,
                ),
                TransparencyMode::NonCustomizable(NonCustomizableTransparencyMode::Vocal),
            ]
            .into(),
            has_normal_mode: true,
        }
    }

    pub fn has_normal_mode(&self) -> bool {
        self.has_normal_mode
    }

    pub fn allowed_anc_modes(&self) -> &[ANCMode] {
        &self.allowed_anc_modes
    }
    pub fn allowed_transparency_modes(&self) -> &[TransparencyMode] {
        &self.allowed_transparency_modes
    }

    pub fn has_customizable_transparency(&self) -> Option<TransparencyMode> {
        self.allowed_transparency_modes
            .iter()
            .find(|mode| Self::TRANS_MODES_WITH_CUSTOM_VALUE.contains(mode))
            .cloned()
    }

    pub fn has_customizable_anc(&self) -> Option<ANCMode> {
        self.allowed_anc_modes
            .iter()
            .find(|mode| Self::ANC_MODES_WITH_CUSTOM_VALUE.contains(mode))
            .cloned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_true_when_has_normal_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert!(sound_mode_features.has_normal_mode());
    }

    #[test]
    fn should_return_false_when_does_not_have_normal_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            false,
        );

        assert!(!sound_mode_features.has_normal_mode());
    }

    #[test]
    fn should_return_allowed_anc_modes() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert_eq!(
            sound_mode_features.allowed_anc_modes(),
            &vec![ANCMode::Adaptive(AdaptiveANCMode::Adaptive)]
        );
    }

    #[test]
    fn should_return_allowed_transparency_modes() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert_eq!(
            sound_mode_features.allowed_transparency_modes(),
            &vec![TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent
            )]
        );
    }

    #[test]
    fn should_return_customizable_transparency_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[
                TransparencyMode::Customizable(CustomizableTransparencyMode::TalkMode),
                TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
            ],
            true,
        );

        assert_eq!(
            sound_mode_features.has_customizable_transparency(),
            Some(TransparencyMode::Customizable(
                CustomizableTransparencyMode::Custom
            ))
        );
    }

    #[test]
    fn should_return_none_when_does_not_have_customizable_transparency_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert_eq!(sound_mode_features.has_customizable_transparency(), None);
    }

    #[test]
    fn should_return_customizable_anc_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[
                ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
                ANCMode::Adaptive(AdaptiveANCMode::Custom),
            ],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert_eq!(
            sound_mode_features.has_customizable_anc(),
            Some(ANCMode::Adaptive(AdaptiveANCMode::Custom))
        );
    }

    #[test]
    fn should_return_none_when_does_not_have_customizable_anc_mode() {
        let sound_mode_features = SoundModeFeatures::new(
            &[ANCMode::Adaptive(AdaptiveANCMode::Adaptive)],
            &[TransparencyMode::NonCustomizable(
                NonCustomizableTransparencyMode::FullyTransparent,
            )],
            true,
        );

        assert_eq!(sound_mode_features.has_customizable_anc(), None);
    }
}
