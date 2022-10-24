import { Card, CardContent, Grid, Typography } from "@mui/material";
import useDeviceStore from "../hooks/useDeviceStore";
import A3951ImgLeft from "./../assets/a3951_img_device_left.webp";
import A3951ImgRight from "./../assets/a3951_img_device_right_edited.webp";


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

export default function A3951InfoCard() {
    const { batteryLevel, batteryCharging } = useDeviceStore();
    return (
        <Card sx={{ minWidth: 275, margin: 2 }}>
            <CardContent>
                <Grid container spacing={2} justifyContent="center" alignItems="center">
                    <GridItem img={A3951ImgLeft} infoAlign="left" batteryLevel={batteryLevel.left} batteryCharging={batteryCharging.left} />
                    <GridItem img={A3951ImgRight} infoAlign="right" batteryLevel={batteryLevel.right}  batteryCharging={batteryCharging.right}/>
                </Grid>
            </CardContent>
            {/* <CardActions>
          <Button size="small">Learn More</Button>
        </CardActions> */}
        </Card>
    )
}

function GridItem(props: any) {
    return (
        <Grid item>
            <Grid container spacing={1} justifyContent="center" alignItems="center">
                {props.infoAlign == "left" &&
                    <EarpiceInfo batteryLevel={props.batteryLevel} batteryCharging={props.batteryCharging}/>
                }
                <Grid item>
                    <img src={props.img} alt="A3951" height={80} />
                </Grid>
                {props.infoAlign == "right" &&
                    <EarpiceInfo batteryLevel={props.batteryLevel} batteryCharging={props.batteryCharging} />
                }
            </Grid>
        </Grid>
    )
}

function EarpiceInfo({ batteryLevel, batteryCharging }: any) {
    let icon = null;
    //0-5
    switch (batteryLevel) {
        case 0:
            icon =  !batteryCharging? <BatteryAlertIcon  /> : <BatteryCharging20Icon  />;
            break;
        case 1:
            icon =  !batteryCharging? <Battery20Icon  /> : <BatteryCharging20Icon  />;
            break;
        case 2:
            icon =  !batteryCharging? <Battery30Icon /> : <BatteryCharging30Icon />;
            break;
        case 3:
            icon =  !batteryCharging? <Battery60Icon  /> : <BatteryCharging60Icon  />;
            break;
        case 4:
            icon = !batteryCharging? <Battery80Icon /> : <BatteryCharging80Icon  />;
            break;
        case 5:
            icon = !batteryCharging? <BatteryFullIcon  /> : <BatteryChargingFullIcon />;
            break;
        default:
            icon = <BatteryUnknownIcon />;
            break;
    }

    return (
        <Grid item>
            {icon}
            <Typography variant="body2" color="text.secondary">
                {batteryLevel*2*10}%
            </Typography>
        </Grid>
    )
}