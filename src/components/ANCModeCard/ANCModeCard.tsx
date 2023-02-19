import { ANCModes } from "../../bindings/ANCModes";
import { DeviceSelection } from "../../bindings/DeviceSelection";
import { useDeviceModel } from "../../hooks/useSoundcoreDevice";
import BaseANCModeCard, { ANCSliderProps } from "./base";

export default function ANCModeCard() {
    let { data: model } = useDeviceModel();
    let Component = (props: ANCSliderProps) => <div />;
    let ancButtons: Array<[string, ANCModes | "AncCustomValue"]> = [];
    let transButtons: Array<[string, ANCModes]> = [];

    switch (model) {
        case "A3935":
        case "A3951":
            Component = BaseANCModeCard;
            ancButtons =
                [
                    ["Transport", "AncTransportMode"],
                    ["Outdoor", "AncOutdoorMode"],
                    ["Indoor", "AncIndoorMode"],
                    ["Custom", "AncCustomValue"]
                ];
            transButtons = [
                ["Fully Trasparent", "TransparencyFullyTransparentMode"],
                ["Vocal Mode", "TransparencyVocalMode"]
            ];
            break;
        case "A3027":
        case "A3028":
        case "A3029":
            Component = BaseANCModeCard;
            ancButtons =
                [
                    ["Transport", "AncTransportMode"],
                    ["Outdoor", "AncOutdoorMode"],
                    ["Indoor", "AncIndoorMode"],
                ];
            transButtons = [["Fully Trasparent", "TransparencyFullyTransparentMode"]];
            break;
        default:
            Component = () => <div>Not implemented</div>;
            break;
    }

    console.log("Model: " + model);

    return (
        <Component ancModes={ancButtons} transModes={transButtons} />
    )
}