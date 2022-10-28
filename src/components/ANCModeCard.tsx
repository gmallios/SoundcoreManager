import { Box, Button, Card, CardContent, Divider, Grid, Icon, Stack, styled } from "@mui/material";
import { borderRadius } from "@mui/system";
import { useState } from "react";
import ANCIcon from "../assets/ambient_icon_anc.png";
import NormalIcon from "../assets/ambient_icon_off.png";
import TransIcon from "../assets/ambient_icon_trans.png";

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
    position: AllowedSliderPositions;
}

const ANCSliderSwitcher = styled("div", {
    shouldForwardProp: (prop) => prop !== "position",
})<ANCSliderSwitcherProps>(({ position, theme }) => ({
    /* Remove border radius for miiddle item and animate it */
    zIndex: 2,
    display: "flex",
    flexDirection: "row",
    position: "absolute",
    backgroundColor: theme.palette.primary.dark,
    borderRadius: 28,
    height: 53,
    alignItems: "center",
    justifyContent: "center",
    width: Metrics.switchWidth,
    elevation: 4,
    shadowColor: "black",
    shadowRadius: 10,
    shadowOpacity: 0.31,
    transition: "right 0.32s cubic-bezier(0.87, 0, 0.13, 1)",
    ...(position == "left" && {
        right: Metrics.switchWidth + (Metrics.switchWidth - 49) // 33 + 16
    }),
    ...(position == "right" && {
        right: Metrics.switchWidth - (Metrics.switchWidth - 33) // 16 * 2 padding + 1
    }),
    ...(position == "center" && {
        right: Metrics.switchWidth,
    }),

}));

const ANCSliderButton = styled(Button, {
    shouldForwardProp: (prop) => prop !== "position",
})<ANCSliderSwitcherProps>(({ theme, position }) => ({
    display: "flex",
    flex: 1,
    width: Metrics.containerWidth / 3,
    height: 54,
    justifyContent: "center",
    alignItems: "center",
    ...(position == "left" && {
        borderRadius: "28px 0 0 28px",
    }),
    ...(position == "center" && {
        borderRadius: "0 0 0 0",
    }),
    ...(position == "right" && {
        borderRadius: "0 28px 28px 0",
    }),
}));


export default function ANCModeCard() {

    let [sliderPosition, setSliderPosition] = useState<AllowedSliderPositions>("center");
    let [sliderIcon, setSliderIcon] = useState<string>(NormalIcon);

    return (
        <Card sx={{ minWidth: 275, margin: 1.5, marginLeft: 2, marginRight: 2 }}>
            {/* display: "flex", justifyContent: "center", alignItems: "center", */}
            <CardContent sx={{ '&:last-child': { pb: "16px" } }}>
                <ANCSliderContainer>
                    <ANCSliderSwitcher position={sliderPosition}>
                        <Icon sx={{ display: "flex", width: 32, height: 32, zIndex: 0 }}>
                            <img src={sliderIcon} height="32" />
                        </Icon>
                    </ANCSliderSwitcher>
                    <ANCCardButton setSliderIcon={setSliderIcon} setSliderPosition={setSliderPosition} position="left" icon={ANCIcon} />
                    <ANCCardButton setSliderIcon={setSliderIcon} setSliderPosition={setSliderPosition} position="center" icon={NormalIcon} />
                    <ANCCardButton setSliderIcon={setSliderIcon} setSliderPosition={setSliderPosition} position="right" icon={TransIcon} />
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

function ANCCardButton({ setSliderIcon, setSliderPosition, position, icon }: { setSliderIcon: React.Dispatch<React.SetStateAction<string>>, setSliderPosition: React.Dispatch<React.SetStateAction<AllowedSliderPositions>>, position: AllowedSliderPositions, icon: string }) {
    return (
        <ANCSliderButton position={position} variant="text" onClick={() => { setSliderPosition(position); setSliderIcon(icon) }}>
            <Icon sx={{ display: "flex", width: 32, height: 32, zIndex: 0 }}>
                <img src={icon} height="32" />
            </Icon>
        </ANCSliderButton>
    );
}