import { SupportedModels } from "../types/soundcore-lib";

import A3027Img from "./../assets/a3027_img_device.webp";
import A3028Img from "./../assets/a3028_img_device.webp";
import A3029Img from "./../assets/a3029_img_device.webp";
import A3040Img from "./../assets/a3040_img_device.webp";
import A3935Img from "./../assets/a3935_img_device.webp";
import A3951Img from "./../assets/a3951_img_device.webp";
import A3951LImg from "./../assets/a3951_img_device_left.webp";
import A3951RImg from "./../assets/a3951_img_device_right_edited.webp";

interface DeviceImageProps {
    device: SupportedModels;
    class?: string;
}

const useDeviceImage = (props: DeviceImageProps) => {
    switch (props.device) {
        case "A3951":
            return <img
                src={A3951Img}
                className={props.class}
                alt="device"
                width={130}
                height={120}
            />;
        case "A3027":
            return <img
                src={A3027Img}
                className={props.class}
                alt="device"
                width={145}
                height={125}
            />;
        case "A3028":
            return <img
                src={A3028Img}
                className={props.class}
                alt="device"
                width={145}
                height={125}
            />;
        case "A3029":
            return <img
                src={A3029Img}
                className={props.class}
                alt="device"
                width={145}
                height={125}
            />;
        case "A3040":
            return <img
                src={A3040Img}
                className={props.class}
                alt="device"
                width={145}
                height={125}
            />;
        case "A3935":
            return <img
                src={A3935Img}
                className={props.class}
                alt="device"
                width={145}
                height={125}
            />;
        default:
            return "";

    }

}

export default useDeviceImage;