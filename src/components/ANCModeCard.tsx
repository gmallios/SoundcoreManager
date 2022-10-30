import { Button, Collapse, Grid, Icon, Paper, Slider, Stack, styled } from "@mui/material";
import { Children, ReactNode, useEffect, useState } from "react";
import ANCIcon from "../assets/ambient_icon_anc.png";
import NormalIcon from "../assets/ambient_icon_off.png";
import TransIcon from "../assets/ambient_icon_trans.png";
import useDeviceStore from "../hooks/useDeviceStore";
import { ANCModes } from "../bindings/ANCModes";

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

    const { sendANCMode } = useDeviceStore();

    let [sliderPosition, setSliderPosition] = useState<AllowedSliderPositions>("center");
    let [sliderIcon, setSliderIcon] = useState<string>(NormalIcon);

    let [ancModeSelected, setAncModeSelected] = useState<ANCModes | "AncCustomValue">("AncOutdoorMode"); 
    let [transModeSelected, setTransModeSelected] = useState<ANCModes>("TransparencyFullyTransparentMode");
    let [ancCustomValue, setAncCustomValue] = useState<number | number[]>(10);


    let ancButtons: Array<[string, ANCModes | "AncCustomValue"]> =
        [
            ["Transport", "AncTransportMode"],
            ["Outdoor", "AncOutdoorMode"],
            ["Indoor", "AncIndoorMode"],
            ["Custom", "AncCustomValue"]
        ];

    let transButtons: Array<[string, ANCModes]> =
        [
            ["Fully Trasparent", "TransparencyFullyTransparentMode"],
            ["Vocal Mode", "TransparencyVocalMode"]
        ]

    // TODO: Load persisted values
    useEffect(() => {
        if (sliderPosition == "center"){
            sendANCMode("NormalMode");
        } else if(sliderPosition == "left" && ancModeSelected != "AncCustomValue"){
            sendANCMode(ancModeSelected);
        } else if(sliderPosition == "right"){
            sendANCMode(transModeSelected);
        }
    }, [sliderPosition]);

    useEffect(() => {
        if(sliderPosition == "left"){
            if(ancModeSelected == "AncCustomValue"){
                sendANCMode({ AncCustomValue: ancCustomValue as number });
            } else {
                sendANCMode(ancModeSelected);
            }
        }
    }, [ancModeSelected, ancCustomValue]);

    useEffect(() => {
        if(sliderPosition == "right"){
            sendANCMode(transModeSelected);
        }
    }, [transModeSelected]);

    return (
        <Paper elevation={0} sx={{ marginTop: 1, marginBottom: 1, display: "flex", minWidth: 275, justifyContent: "center", alignItems: "center" }}>
            <Grid sx={{ paddingLeft: 0, justifyContent: "center" }}>
                <Grid item>
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
                </Grid>
                <Grid item sx={{ paddingTop: "0px !important" }}>
                    <Collapse in={sliderPosition != "center"} mountOnEnter unmountOnExit>
                        <ButtonGrid buttonArray={sliderPosition == "left" ? ancButtons : transButtons} setButtonSelected={sliderPosition == "left" ? setAncModeSelected : setTransModeSelected} buttonSelected={sliderPosition == "left" ? ancModeSelected : transModeSelected}>
                            <Collapse in={sliderPosition == "left" && ancModeSelected == "AncCustomValue"}>
                                <Slider
                                    size="small"
                                    defaultValue={10}
                                    onChange={(_, newValue) => setAncCustomValue(newValue)}
                                    onChangeCommitted={(_, newValue) => setAncCustomValue(newValue)}
                                    sx={{ mt: 2, pb: 0, width: "98%" }}
                                    min={0}
                                    max={10}
                                    marks
                                    aria-label="Small"
                                    valueLabelDisplay="auto"
                                />
                            </Collapse>
                        </ButtonGrid>
                    </Collapse>
                </Grid>
            </Grid>
        </Paper>
    )
};

enum ANCCardButtonVariant {
    NOISE_CANCELLING,
    NORMAL_MODE,
    TRANSPARENCY_MODE
}

function ANCCardButton({ setSliderIcon, setSliderPosition, position, icon }: { setSliderIcon: React.Dispatch<React.SetStateAction<string>>, setSliderPosition: React.Dispatch<React.SetStateAction<AllowedSliderPositions>>, position: AllowedSliderPositions, icon: string }) {
    return (
        <ANCSliderButton position={position} variant="text" onClick={() => { setSliderPosition(position); setSliderIcon(icon); }}>
            <Icon sx={{ display: "flex", width: 32, height: 32, zIndex: 0 }}>
                <img src={icon} height="32" />
            </Icon>
        </ANCSliderButton>
    );
}

interface ANCModeButtonProps {
    active?: boolean
}

const ANCModeButton = styled(Button, {
    shouldForwardProp: (prop) => prop !== "active",
})<ANCModeButtonProps>(({ theme, active }) => ({
    //width: "100px",
    backgroundColor: active ? theme.palette.primary.dark : "transparent",
    color: active ? theme.palette.text.primary : theme.palette.text.secondary,
}));


function ButtonGrid({ children, buttonArray, setButtonSelected, buttonSelected }: { children: ReactNode, buttonArray: Array<[string, ANCModes | "AncCustomValue"]>, setButtonSelected: React.Dispatch<React.SetStateAction<any>>, buttonSelected: any }) {
    return (
        <Stack>
            <Grid container direction="row" spacing={1} sx={{ display: "flex", justifyContent: "space-evenly", pt: 2 }}>
                {buttonArray.map(([title, mode]) => (
                    <Grid item key={title}><ANCModeButton variant="outlined" active={buttonSelected == mode} onClick={() => { setButtonSelected(mode) }} size="small">{title}</ANCModeButton></Grid>
                ))}
            </Grid>
            {children}
        </Stack>);
}