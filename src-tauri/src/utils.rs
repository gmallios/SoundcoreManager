use crate::frontend_types::ANCModes;
use soundcore_lib::types::ANCProfile;

// TODO: ANCModes should carry the saved CustomValue as well in order to send it
// TODO: Cleanup ANC types
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
        ANCProfile{ option: 0, anc_option: 0, .. } => ANCModes::AncTransportMode,
        ANCProfile{ option: 0, anc_option: 1, .. } => ANCModes::AncOutdoorMode,
        ANCProfile{ option: 0, anc_option: 2, .. } => ANCModes::AncIndoorMode,
        ANCProfile{ option: 0, anc_option: 3, .. } => ANCModes::AncCustomValue(profile.anc_custom),
        ANCProfile{ option: 1, transparency_option: 0, .. } => ANCModes::TransparencyFullyTransparentMode,
        ANCProfile{ option: 1, transparency_option: 1, .. } => ANCModes::TransparencyVocalMode,
        ANCProfile{ option: 2, .. } => ANCModes::NormalMode,
        _ => unreachable!(),
    }
}
