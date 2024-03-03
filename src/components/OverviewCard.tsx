import { Box, Grid, Paper, Typography } from '@mui/material';
import A3951ImgLeft from './../assets/a3951_img_device_left.webp';
import A3951ImgRight from './../assets/a3951_img_device_right_edited.webp';
import A3027Img from './../assets/a3027_img_device.webp';
import A3028Img from './../assets/a3028_img_device.webp';
import A3029Img from './../assets/a3029_img_device.webp';
import A3040Img from './../assets/a3040_img_device.webp';
import A3935Img from './../assets/a3935_img_device.webp';

import BatteryAlertIcon from '@mui/icons-material/BatteryAlert';
import Battery20Icon from '@mui/icons-material/Battery20';
import BatteryCharging20Icon from '@mui/icons-material/BatteryCharging20';
import Battery30Icon from '@mui/icons-material/Battery30';
import BatteryCharging30Icon from '@mui/icons-material/BatteryCharging30';
import Battery60Icon from '@mui/icons-material/Battery60';
import BatteryCharging60Icon from '@mui/icons-material/BatteryCharging60';
import Battery80Icon from '@mui/icons-material/Battery80';
import BatteryCharging80Icon from '@mui/icons-material/BatteryCharging80';
import BatteryFullIcon from '@mui/icons-material/BatteryFull';
import BatteryChargingFullIcon from '@mui/icons-material/BatteryChargingFull';
import BatteryUnknownIcon from '@mui/icons-material/BatteryUnknown';
import { useBatteryLevel, useCharging, useDeviceModel } from '../hooks/useSoundcoreDevice';

export default function OverviewCard() {
  // if(levelQuery.isLoading || chargingQuery.isLoading) {
  //     return(
  //         <div></div>
  //     );
  // }

  return (
    // As card
    // <Card sx={{ minWidth: 275, margin: 1.5 }}>
    //     <CardContent>
    //         <Grid container spacing={2} justifyContent="center" alignItems="center">
    //             <EarbudItem img={A3951ImgLeft} alignTo="left" batteryLevel={batteryLevel.left} batteryCharging={batteryCharging.left} />
    //             <EarbudItem img={A3951ImgRight} alignTo="right" batteryLevel={batteryLevel.right}  batteryCharging={batteryCharging.right}/>
    //         </Grid>
    //     </CardContent>
    //     {/* <CardActions>
    //   <Button size="small">Learn More</Button>
    // </CardActions> */}
    // </Card>
    <Box sx={{ display: 'block', maxWidth: '300px', margin: 'auto', mb: 0 }}>
      <Paper
        sx={{ display: 'flex', margin: 1.5, justifyContent: 'center', alignItems: 'center' }}
        elevation={0}>
        <OverviewItem />
      </Paper>
    </Box>
  );
}

function OverviewItem() {
  const chargingQuery = useCharging();
  const levelQuery = useBatteryLevel();
  const { data: deviceModel } = useDeviceModel();

  switch (deviceModel) {
    case 'A3951':
      return (
        <>
          <EarbudItem
            img={A3951ImgLeft}
            imgSize={80}
            alignTo="left"
            batteryLevel={levelQuery.data?.left}
            batteryCharging={chargingQuery.data?.left}
          />
          <EarbudItem
            img={A3951ImgRight}
            imgSize={80}
            alignTo="right"
            batteryLevel={levelQuery.data?.right}
            batteryCharging={chargingQuery.data?.right}
          />
        </>
      );
    case 'A3027':
      return (
        <EarbudItem
          img={A3027Img}
          imgSize={90}
          alignTo="right"
          batteryLevel={levelQuery.data?.left}
          batteryCharging={chargingQuery.data?.left}
        />
      );
    case 'A3028':
      return (
        <EarbudItem
          img={A3028Img}
          imgSize={90}
          alignTo="right"
          batteryLevel={levelQuery.data?.left}
          batteryCharging={chargingQuery.data?.left}
        />
      );
    case 'A3029':
      return (
        <EarbudItem
          img={A3029Img}
          imgSize={90}
          alignTo="right"
          batteryLevel={levelQuery.data?.left}
          batteryCharging={chargingQuery.data?.left}
        />
      );
    case 'A3040':
      return (
        <SingleImgTwoBattery
          img={A3040Img}
          imgSize={90}
          leftBattLevel={levelQuery.data?.left}
          rightBattLevel={levelQuery.data?.right}
          leftCharging={chargingQuery.data?.left}
          rightCharging={chargingQuery.data?.right}
        />
      );
    case 'A3935':
      return (
        <SingleImgTwoBattery
          img={A3935Img}
          imgSize={90}
          leftBattLevel={levelQuery.data?.left}
          rightBattLevel={levelQuery.data?.right}
          leftCharging={chargingQuery.data?.left}
          rightCharging={chargingQuery.data?.right}
        />
      );
    default:
      return <h1>Something went wrong...</h1>;
  }
}

function SingleImgTwoBattery({
  img,
  imgSize,
  leftBattLevel,
  rightBattLevel,
  leftCharging,
  rightCharging
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
}: any) {
  return (
    <Grid item>
      <Grid container spacing={1} justifyContent="center" alignItems="center">
        <EarbudBattery batteryLevel={leftBattLevel} batteryCharging={leftCharging} />
        <Grid item>
          <img src={img} height={imgSize} />
        </Grid>
        <EarbudBattery batteryLevel={rightBattLevel} batteryCharging={rightCharging} />
      </Grid>
    </Grid>
  );
}

function EarbudItem({
  alignTo,
  batteryLevel,
  batteryCharging,
  img,
  imgSize
}: {
  alignTo: 'left' | 'right';
  batteryLevel: number | undefined;
  batteryCharging: boolean | undefined;
  img: string;
  imgSize: number;
}) {
  return (
    <Grid item>
      <Grid container spacing={1} justifyContent="center" alignItems="center">
        {alignTo == 'left' && (
          <EarbudBattery batteryLevel={batteryLevel} batteryCharging={batteryCharging} />
        )}
        <Grid item>
          <img src={img} height={imgSize} />
        </Grid>
        {alignTo == 'right' && (
          <EarbudBattery batteryLevel={batteryLevel} batteryCharging={batteryCharging} />
        )}
      </Grid>
    </Grid>
  );
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function EarbudBattery({ batteryLevel, batteryCharging }: any) {
  let icon = null;
  //0-5
  switch (batteryLevel) {
    case 0:
      icon = !batteryCharging ? <BatteryAlertIcon /> : <BatteryCharging20Icon />;
      break;
    case 1:
      icon = !batteryCharging ? <Battery20Icon /> : <BatteryCharging20Icon />;
      break;
    case 2:
      icon = !batteryCharging ? <Battery30Icon /> : <BatteryCharging30Icon />;
      break;
    case 3:
      icon = !batteryCharging ? <Battery60Icon /> : <BatteryCharging60Icon />;
      break;
    case 4:
      icon = !batteryCharging ? <Battery80Icon /> : <BatteryCharging80Icon />;
      break;
    case 5:
      icon = !batteryCharging ? <BatteryFullIcon /> : <BatteryChargingFullIcon />;
      break;
    default:
      icon = <BatteryUnknownIcon />;
      break;
  }

  return (
    <Grid item>
      {icon}
      <Typography variant="body2" color="text.secondary">
        {batteryLevel * 2 * 10}%
      </Typography>
    </Grid>
  );
}
