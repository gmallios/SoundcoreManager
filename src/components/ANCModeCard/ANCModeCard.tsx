import { ANCModes } from "../../bindings/ANCModes";
import { DeviceSelection } from "../../bindings/DeviceSelection";
import BaseANCModeCard, { ANCSliderProps } from "./base";

export default function ANCModeCard({ model }: { model: DeviceSelection }) {
    let Component = (props: ANCSliderProps) => <div />;
    let ancButtons: Array<[string, ANCModes | "AncCustomValue"]> = [];
    let transButtons: Array<[string, ANCModes]> = [];

    switch (model) {
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