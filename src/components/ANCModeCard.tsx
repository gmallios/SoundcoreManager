import { Box, Button, Card, CardContent, Divider, Grid, Stack, styled } from "@mui/material";
import { useState } from "react";


const width = 465;

const Metrics = {
    containerWidth: width - 30,
    switchWidth: width / 2.7,
}

const ANCSliderContainer = styled("div")(({ theme }) => ({
    width: Metrics.containerWidth,
    height: 55,
    display: "flex",
    flexDirection: "row",
    backgroundColor: theme.palette.background.paper,
    alignItems: "center",
    justifyContent: "center",
    borderWidth: 1,
    borderColor: theme.palette.divider,
    borderStyle: "solid",
    borderRadius: 27.5,
}));

type AllowedSliderPositions = "left" | "right" | "center";

interface ANCSliderSwitcherProps {
    position?: AllowedSliderPositions;
}

const ANCSliderSwitcher = styled("div", {
    shouldForwardProp: (prop) => prop !== "position",
})<ANCSliderSwitcherProps>(({ position, theme }) => ({
    display: "flex",
    flexDirection: "row",
    position: "absolute",
    backgroundColor: theme.palette.primary.main,
    borderRadius: 28,
    height: 53,
    alignItems: "center",
    justifyContent: "center",
    width: Metrics.switchWidth,
    elevation: 4,
    shadowColor: "black",
    shadowRadius: 10,
    shadowOpacity: 0.31,
    transition: "right 0.65s",
    ...(position == "left" && {
        right: Metrics.switchWidth + (Metrics.switchWidth-49) // 33 + 16
    }),
    ...(position == "right" && {
        right: Metrics.switchWidth - (Metrics.switchWidth-33) // 16 * 2 padding + 1
    }),
    ...(position == "center" && {
        right: Metrics.switchWidth
    }),

}));

const ANCSliderButton = styled("div")(({ theme }) => ({
    display: "flex",
    flex: 1,
    width: Metrics.containerWidth / 3,
    height: 54,
    justifyContent: "center",
    alignItems: "center",
}));


export default function ANCModeCard() {

    let [sliderPosition, setSliderPosition] = useState<AllowedSliderPositions>("center");
    

    return (
        <Card sx={{ minWidth: 275, margin: 1.5, marginLeft: 2, marginRight: 2 }}>
            {/* display: "flex", justifyContent: "center", alignItems: "center", */}
            <CardContent sx={{ '&:last-child': { pb: "16px" }}}>
                <ANCSliderContainer>
                    <ANCSliderSwitcher position={sliderPosition}><a>S</a></ANCSliderSwitcher>
                    <ANCCardButton setSliderPosition={setSliderPosition} position="left" />
                    <ANCCardButton setSliderPosition={setSliderPosition} position="center" />
                    <ANCCardButton setSliderPosition={setSliderPosition} position="right" />
                </ANCSliderContainer>
            </CardContent>
        </Card>
    )
};

enum ANCCardButtonVariant {
    NOISE_CANCELLING,
    NORMAL_MODE,
    TRANSPARENCY_MODE
}

function ANCCardButton({setSliderPosition, position}: {setSliderPosition: React.Dispatch<React.SetStateAction<AllowedSliderPositions>>, position: AllowedSliderPositions}) {
    return (
        <ANCSliderButton>
        <p onClick={() => { setSliderPosition(position)}}>A</p>
        </ANCSliderButton>
    );
}