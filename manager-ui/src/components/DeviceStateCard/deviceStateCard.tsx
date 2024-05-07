import { Battery } from '@generated-types/soundcore-lib';
import { Box, Grid, Paper } from '@mui/material';
import { useSoundcoreStore } from '@stores/useSoundcoreStore';
import { getImageForModel } from '@utils/modelToImgMap';
import { BatteryIcon } from './batteryIcon';

export const DeviceStateCard: React.FC = () => {
  const currentState = useSoundcoreStore((state) => state.currentViewedDeviceState());

  return (
    <>
      <Box sx={{ display: 'block', maxWidth: '300px', margin: 'auto', mb: 0 }}>
        <Paper
          sx={{ display: 'flex', margin: 1.5, justifyContent: 'center', alignItems: 'center' }}
          elevation={0}
        >
          <ProductImageWithBattery
            model={currentState?.serial?.model}
            battery={currentState?.battery}
          />
        </Paper>
      </Box>
    </>
  );
};

const ProductImageWithBattery: React.FC<{
  model: string | null | undefined;
  battery: Battery | undefined;
}> = ({ model, battery }) => {
  if (!battery) {
    // TODO: Handle unknown battery state, shouldn't ever happen
    return <></>;
  }

  if (battery?.type == 'single') {
    return (
      <Grid>
        <Grid container spacing={1} justifyContent="center" alignItems="center">
          <ProductImage model={model} />
          <BatteryIcon battery={battery.value} />
        </Grid>
      </Grid>
    );
  }

  if (battery?.type == 'dual') {
    return (
      <Grid>
        <Grid container spacing={1} justifyContent="center" alignItems="center">
          <BatteryIcon battery={battery.value.left} />
          <ProductImage model={model} />
          <BatteryIcon battery={battery.value.right} />
        </Grid>
      </Grid>
    );
  }

  return (
    <Grid>
      <Grid container spacing={1} justifyContent="center" alignItems="center">
        <ProductImage model={model} />
      </Grid>
    </Grid>
  );
};

const ProductImage: React.FC<{ model: string | null | undefined }> = ({ model }) => {
  const imageResult = getImageForModel(model || '');

  if (!imageResult) {
    return <></>;
  }

  return (
    <>
      {imageResult && imageResult.kind === 'single' ? (
        <img
          src={imageResult.data.img}
          width={imageResult.data.width}
          height={imageResult.data.height}
        />
      ) : (
        <>
          <img
            src={imageResult.data.left.img}
            width={imageResult.data.left.width}
            height={imageResult.data.left.height}
            draggable={false}
          />
          <img
            src={imageResult.data.right.img}
            width={imageResult.data.right.width}
            height={imageResult.data.right.height}
          />
        </>
      )}
    </>
  );
};
