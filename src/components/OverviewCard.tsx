import { Box, Card, CardContent, Grid, Paper, Typography } from "@mui/material";
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
import { useBatteryLevel, useCharging } from "../hooks/useSoundcoreDevice";

export default function OverviewCard() {
    const chargingQuery = useCharging();
    const levelQuery = useBatteryLevel();

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
        <Box sx={{display: "block", pt: 3, maxWidth: "300px", margin: "auto"}}>
            <Paper sx={{ display: "flex", margin: 1.5, justifyContent: "center", alignItems: "center"}} elevation={0}>
                <EarbudItem img={A3951ImgLeft} alignTo="left" batteryLevel={levelQuery.data?.left} batteryCharging={chargingQuery.data?.left} />
                <EarbudItem img={A3951ImgRight} alignTo="right" batteryLevel={levelQuery.data?.right} batteryCharging={chargingQuery.data?.right} />
            </Paper>
        </Box>
    )
}

function EarbudItem({ alignTo, batteryLevel, batteryCharging, img }: { alignTo: "left" | "right", batteryLevel: number | undefined, batteryCharging: boolean | undefined, img: string }) {
    return (
        <Grid item>
            <Grid container spacing={1} justifyContent="center" alignItems="center">
                {alignTo == "left" &&
                    <EarpiceInfo batteryLevel={batteryLevel} batteryCharging={batteryCharging} />
                }
                <Grid item>
                    <img src={img} alt="A3951" height={80} />
                </Grid>
                {alignTo == "right" &&
                    <EarpiceInfo batteryLevel={batteryLevel} batteryCharging={batteryCharging} />
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
    )
}