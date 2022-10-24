import { Card, CardContent, Grid, Typography } from "@mui/material";
import useDeviceStore from "../hooks/useDeviceStore";
import A3951ImgLeft from "./../assets/a3951_img_device_left.webp";
import A3951ImgRight from "./../assets/a3951_img_device_right.webp";




export default function A3951InfoCard() {
    const { batteryLevel } = useDeviceStore();
    return (
        <Card sx={{ minWidth: 275, margin: 2 }}>
            <CardContent>
                <Grid container spacing={2} justifyContent="center" alignItems="center">
                    <GridItem img={A3951ImgLeft} infoAlign="left" batteryLevel={batteryLevel.left} />
                    <GridItem img={A3951ImgRight} infoAlign="right" batteryLevel={batteryLevel.right} />
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
                    <EarpiceInfo batteryLevel={props.batteryLevel} />
                }
                <Grid item>
                    <img src={props.img} alt="A3951" height={80} />
                </Grid>
                {props.infoAlign == "right" &&
                    <EarpiceInfo batteryLevel={props.batteryLevel} />
                }
            </Grid>
        </Grid>
    )
}

function EarpiceInfo(props: any) {
    return (
        <Grid item>
            <Typography variant="body2" color="text.secondary">
                {props.batteryLevel}
            </Typography>
        </Grid>
    )
}