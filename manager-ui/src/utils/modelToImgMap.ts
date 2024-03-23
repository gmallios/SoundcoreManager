import A3951ImgLeft from '@assets/a3951_img_device_left.webp';
import A3951ImgRight from '@assets/a3951_img_device_right_edited.webp';
import A3027Img from '@assets/a3027_img_device.webp';
import A3028Img from '@assets/a3028_img_device.webp';
import A3029Img from '@assets/a3029_img_device.webp';
import A3040Img from '@assets/a3040_img_device.webp';
import A3935Img from '@assets/a3935_img_device.webp';

export type ImageData = {
  img: string;
  width?: number;
  height?: number;
};

const ModelSingleImageMap: {
  [key: string]: ImageData;
} = {
  A3040: { img: A3040Img, height: 90 },
  A3027: { img: A3027Img, height: 90 },
  A3028: { img: A3028Img, height: 90 },
  A3029: { img: A3029Img, height: 90 },
  A3935: { img: A3935Img, height: 90 }
};

const ModelDoubleImageMap: {
  [key: string]: {
    left: ImageData;
    right: ImageData;
  };
} = {
  A3951: {
    left: { img: A3951ImgLeft, height: 80 },
    right: { img: A3951ImgRight, height: 80 }
  }
};

export type ModelImageMapResult =
  | {
      kind: 'single';
      data: ImageData;
    }
  | {
      kind: 'double';
      data: {
        left: ImageData;
        right: ImageData;
      };
    }
  | null;

export const getImageForModel = (_model: string): ModelImageMapResult => {
  if (ModelSingleImageMap[_model]) {
    return {
      kind: 'single',
      data: ModelSingleImageMap[_model]
    };
  }

  if (ModelDoubleImageMap[_model]) {
    return {
      kind: 'double',
      data: ModelDoubleImageMap[_model]
    };
  }

  return null;
};
