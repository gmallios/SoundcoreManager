import { Battery, SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import { getImageForModel } from '@utils/modelToImgMap';
import { BatteryIcon } from './batteryIcon';
import React from 'react';
import { Card, CardBody, Image } from '@nextui-org/react';
import { getDeviceName } from '@utils/getDeviceName';
import { SoundModeTabs } from '@components/DeviceStateCard/soundModeTabs';

export const DeviceStateCard: React.FC<{
  state: SoundcoreDeviceState | null;
}> = ({ state }) => {
  if (!state) {
    return <></>;
  }

  return (
    <>
      <Card
        isBlurred
        className="border-none bg-background/60 dark:bg-default-100/50 m-5 w-max flex"
        shadow="sm">
        <CardBody>
          <div className="grid grid-cols-6 md:grid-cols-12 gap-8 md:gap-4 items-center justify-center">
            <div className="relative col-span-6 md:col-span-3">
              <ProductImage model={state?.serial?.model} />
            </div>

            <div className="flex flex-col col-span-6 md:col-span-9 ml-2 mt-4 self-start">
              <div className="flex justify-between items-start">
                <div className="flex flex-col gap-0">
                  <h3 className="font-semibold text-foreground/90">
                    {getDeviceName(state?.serial?.model)}
                  </h3>
                  <BatteryRow battery={state?.battery} />
                </div>
              </div>

              <div className="flex w-full items-center justify-center">
                <SoundModeTabs state={state} />
              </div>
            </div>
          </div>
        </CardBody>
      </Card>
    </>
  );
};

const BatteryRow: React.FC<{
  battery: Battery | undefined;
}> = ({ battery }) => {
  if (!battery) {
    // TODO: Handle unknown battery state
    return <></>;
  }

  if (battery?.type == 'single') {
    return (
      <div className={'flex items-center'}>
        <BatteryIcon battery={battery.value} />
      </div>
    );
  }

  if (battery?.type == 'dual') {
    return (
      <div className={'flex items-center'}>
        <BatteryIcon battery={battery.value.left} />
        <BatteryIcon battery={battery.value.right} />
      </div>
    );
  }

  return <></>;
};

const ProductImage: React.FC<{ model: string | null | undefined }> = ({ model }) => {
  const imageResult = getImageForModel(model || '');

  if (!imageResult) {
    return <></>;
  }

  const imageProps: React.ComponentProps<typeof Image> = {
    isBlurred: true,
    width: '100%',
    className: 'object-cover',
    shadow: 'sm',
    style: {
      height: '190px'
    }
  };

  return (
    <>
      {imageResult && imageResult.kind === 'single' ? (
        <Image src={imageResult.data.img} {...imageProps} />
      ) : (
        <>
          <Image src={imageResult.data.left.img} {...imageProps} />
          <Image src={imageResult.data.right.img} {...imageProps} />
        </>
      )}
    </>
  );
};
