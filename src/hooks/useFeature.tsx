import { SupportedModels } from "../types/soundcore-lib";
import { ANCModes, SupportedANCProfiles } from "../types/tauri-backend";

export const useAvailableANCModes = (model: SupportedModels): string[] => {
    let modes;
    switch (model) {
        case "A3027":
        case "A3028":
        case "A3029":
            modes = [SupportedANCProfiles.AncTransportMode,
            SupportedANCProfiles.AncOutdoorMode,
            SupportedANCProfiles.AncIndoorMode,
            SupportedANCProfiles.AncCustomValue
            ];
        default:
            modes = [SupportedANCProfiles.AncTransportMode,
            SupportedANCProfiles.AncOutdoorMode,
            SupportedANCProfiles.AncIndoorMode,
            SupportedANCProfiles.AncCustomValue,
            SupportedANCProfiles.TransparencyFullyTransparentMode,
            SupportedANCProfiles.TransparencyVocalMode
            ];
    }
    return mapAncModesToTitle(modes);
}

const mapAncModesToTitle = (modes: SupportedANCProfiles[]): string[] => {
    return modes.map(mode => {
        switch (mode) {
            case SupportedANCProfiles.TransparencyFullyTransparentMode:
                return "Fully Transparent";
            case SupportedANCProfiles.TransparencyVocalMode:
                return "Vocal";
            case SupportedANCProfiles.AncTransportMode:
                return "Transport";
            case SupportedANCProfiles.AncOutdoorMode:
                return "Outdoor";
            case SupportedANCProfiles.AncIndoorMode:
                return "Indoor";
            case SupportedANCProfiles.AncCustomValue:
                return "Custom";
            default:
                return "Unknown";
        }
    });
}