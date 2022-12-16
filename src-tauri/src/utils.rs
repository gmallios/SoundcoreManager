use crate::frontend_types::ANCModes;
use soundcore_lib::types::ANCProfile;

pub(crate) fn anc_mode_to_profile(mode: ANCModes) -> ANCProfile {
    match mode {
        ANCModes::NormalMode => ANCProfile::NORMAL_MODE,
        ANCModes::AncTransportMode => ANCProfile::ANC_TRANSPORT_MODE,
        ANCModes::AncOutdoorMode => ANCProfile::ANC_OUTDOOR_MODE,
        ANCModes::AncIndoorMode => ANCProfile::ANC_INDOOR_MODE,
        ANCModes::AncCustomValue(value) => ANCProfile::anc_custom_value(value),
        ANCModes::TransparencyFullyTransparentMode => {
            ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE
        }
        ANCModes::TransparencyVocalMode => ANCProfile::TRANSPARENCY_VOCAL_MODE,
    }
}

pub(crate) fn anc_profile_to_mode(profile: ANCProfile) -> ANCModes {
    match profile {
        ANCProfile::ANC_INDOOR_MODE => ANCModes::AncIndoorMode,
        ANCProfile::ANC_OUTDOOR_MODE => ANCModes::AncOutdoorMode,
        ANCProfile::ANC_TRANSPORT_MODE => ANCModes::AncTransportMode,
        ANCProfile::NORMAL_MODE => ANCModes::NormalMode,
        ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE => {
            ANCModes::TransparencyFullyTransparentMode
        }
        ANCProfile::TRANSPARENCY_VOCAL_MODE => ANCModes::TransparencyVocalMode,
        custom_val => ANCModes::AncCustomValue(custom_val.anc_custom),
    }
}
