use crate::frontend_types::ANCModes;
use soundcore_lib::anc_types::ANCRawData;

// TODO: Cleanup ANC types
pub(crate) fn anc_mode_to_raw(mode: ANCModes) -> ANCRawData {
    match mode {
        ANCModes::NormalMode => ANCRawData::NORMAL_MODE,
        ANCModes::AncTransportMode => ANCRawData::ANC_TRANSPORT_MODE,
        ANCModes::AncOutdoorMode => ANCRawData::ANC_OUTDOOR_MODE,
        ANCModes::AncIndoorMode => ANCRawData::ANC_INDOOR_MODE,
        ANCModes::AncCustomValue(value) => ANCRawData::anc_custom_value(value),
        ANCModes::TransparencyFullyTransparentMode => {
            ANCRawData::TRANSPARENCY_FULLY_TRANSPARENT_MODE
        }
        ANCModes::TransparencyVocalMode => ANCRawData::TRANSPARENCY_VOCAL_MODE,
    }
}


pub(crate) fn raw_anc_to_mode(raw_anc: ANCRawData) -> ANCModes {
    match raw_anc {
        ANCRawData{ option: 0, anc_option: 0, .. } => ANCModes::AncTransportMode,
        ANCRawData{ option: 0, anc_option: 1, .. } => ANCModes::AncOutdoorMode,
        ANCRawData{ option: 0, anc_option: 2, .. } => ANCModes::AncIndoorMode,
        ANCRawData{ option: 0, anc_option: 3, .. } => ANCModes::AncCustomValue(raw_anc.anc_custom),
        ANCRawData{ option: 1, transparency_option: 0, .. } => ANCModes::TransparencyFullyTransparentMode,
        ANCRawData{ option: 1, transparency_option: 1, .. } => ANCModes::TransparencyVocalMode,
        ANCRawData{ option: 2, .. } => ANCModes::NormalMode,
        _ => unreachable!(),
    }
}
