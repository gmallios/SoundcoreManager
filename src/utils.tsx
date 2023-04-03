import { SupportedModels } from "./types/soundcore-lib";
import A3951img from "./assets/a3951_img_device.webp";
import A3027Img from "./assets/a3027_img_device.webp";
import A3028Img from "./assets/a3028_img_device.webp";
import A3029Img from "./assets/a3029_img_device.webp";
import A3040Img from "./assets/a3040_img_device.webp";
import A3935Img from "./assets/a3935_img_device.webp";


/* TODO: Add images where the device is shown as a pair (left and right) */
export function getSoundcoreIcon(modelid: SupportedModels): string {
    switch (modelid) {
        case "A3951":
            return A3951img;
        case "A3027":
            return A3027Img;
        case "A3028":
            return A3028Img;
        case "A3029":
            return A3029Img;
        case "A3040":
            return A3040Img;
        case "A3935":
            return A3935Img;
        default:
            return "";
        };

}