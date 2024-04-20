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
    has_normal: bool,
    has_customizable_anc: bool,
    has_customizable_transparency: bool,
    max_custom_anc: Option<u8>,
    max_custom_transparency: Option<u8>,
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
        let has_customizable_anc = Self::has_customizable_anc(anc_modes.into());
        let has_customizable_transparency =
            Self::has_customizable_transparency(transparency_modes.into());
        Self {
            allowed_anc_modes: anc_modes.into(),
            allowed_transparency_modes: transparency_modes.into(),
            has_normal: normal_mode,
            has_customizable_anc: has_customizable_anc.is_some(),
            has_customizable_transparency: has_customizable_transparency.is_some(),
            max_custom_anc: has_customizable_anc.and_then(Self::determine_max_custom_anc),
            max_custom_transparency: has_customizable_transparency
                .and_then(Self::determine_max_custom_transparency),
        }
    }

    pub fn adaptive_customizable_anc_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures::new(
            &[
                ANCMode::Adaptive(AdaptiveANCMode::Custom),
                ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
            ],
            &[
                TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
                TransparencyMode::Customizable(CustomizableTransparencyMode::TalkMode),
            ],
            true,
        )
    }

    pub fn scene_based_customizable_anc_non_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures::new(
            &[
                ANCMode::SceneBased(SceneBasedANCMode::Custom),
                ANCMode::SceneBased(SceneBasedANCMode::Indoor),
                ANCMode::SceneBased(SceneBasedANCMode::Outdoor),
                ANCMode::SceneBased(SceneBasedANCMode::Transport),
            ],
            &[
                TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::FullyTransparent,
                ),
                TransparencyMode::NonCustomizable(NonCustomizableTransparencyMode::Vocal),
            ],
            true,
        )
    }

    pub fn scene_based_non_customizable_anc_non_customizable_transparency() -> SoundModeFeatures {
        SoundModeFeatures::new(
            &[
                ANCMode::SceneBased(SceneBasedANCMode::Indoor),
                ANCMode::SceneBased(SceneBasedANCMode::Outdoor),
                ANCMode::SceneBased(SceneBasedANCMode::Transport),
            ],
            &[
                TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::FullyTransparent,
                ),
                TransparencyMode::NonCustomizable(NonCustomizableTransparencyMode::Vocal),
            ],
            true,
        )
    }

    pub fn has_normal_mode(&self) -> bool {
        self.has_normal
    }

    pub fn allowed_anc_modes(&self) -> &[ANCMode] {
        &self.allowed_anc_modes
    }
    pub fn allowed_transparency_modes(&self) -> &[TransparencyMode] {
        &self.allowed_transparency_modes
    }

    pub fn has_customizable_transparency(
        modes: Arc<[TransparencyMode]>,
    ) -> Option<TransparencyMode> {
        modes
            .iter()
            .find(|mode| Self::TRANS_MODES_WITH_CUSTOM_VALUE.contains(mode))
            .cloned()
    }

    pub fn has_customizable_anc(modes: Arc<[ANCMode]>) -> Option<ANCMode> {
        modes
            .iter()
            .find(|mode| Self::ANC_MODES_WITH_CUSTOM_VALUE.contains(mode))
            .cloned()
    }

    pub fn determine_max_custom_anc(mode: ANCMode) -> Option<u8> {
        match mode {
            ANCMode::Adaptive(AdaptiveANCMode::Custom) => Some(5),
            ANCMode::SceneBased(SceneBasedANCMode::Custom) => Some(10),
            _ => None,
        }
    }

    pub fn determine_max_custom_transparency(mode: TransparencyMode) -> Option<u8> {
        match mode {
            TransparencyMode::Customizable(CustomizableTransparencyMode::Custom) => Some(5),
            _ => None,
        }
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
        assert_eq!(
            SoundModeFeatures::has_customizable_transparency(
                [
                    TransparencyMode::Customizable(CustomizableTransparencyMode::TalkMode),
                    TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
                ]
                .into()
            ),
            Some(TransparencyMode::Customizable(
                CustomizableTransparencyMode::Custom
            ))
        );
    }

    #[test]
    fn should_return_none_when_does_not_have_customizable_transparency_mode() {
        assert_eq!(
            SoundModeFeatures::has_customizable_transparency(
                [TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::FullyTransparent,
                )]
                .into()
            ),
            None
        );
    }

    #[test]
    fn should_return_customizable_anc_mode() {
        assert_eq!(
            SoundModeFeatures::has_customizable_anc(
                [
                    ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
                    ANCMode::Adaptive(AdaptiveANCMode::Custom),
                ]
                .into()
            ),
            Some(ANCMode::Adaptive(AdaptiveANCMode::Custom))
        );
    }

    #[test]
    fn should_return_none_when_does_not_have_customizable_anc_mode() {
        assert_eq!(
            SoundModeFeatures::has_customizable_anc(
                [ANCMode::Adaptive(AdaptiveANCMode::Adaptive)].into()
            ),
            None
        );
    }
}
