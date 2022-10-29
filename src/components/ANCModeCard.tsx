import { Button, Collapse, Grid, Icon, Paper, Slider, Stack, styled } from "@mui/material";
import { useEffect, useState } from "react";
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

    const { setANCMode } = useDeviceStore();

    let [sliderPosition, setSliderPosition] = useState<AllowedSliderPositions>("center");
    let [sliderIcon, setSliderIcon] = useState<string>(NormalIcon);
    let [ancModeSelected, setAncModeSelected] = useState<ANCModes>("NormalMode"); // Keep track across restars?


    // TODO: Load persisted values
    useEffect(() => {
        // Handle middle position
        if(sliderPosition == "center")
            setANCMode("NormalMode");
    }, [sliderPosition]);

    useEffect(() => {
        // Handles all other positions
        setANCMode(ancModeSelected);
    }, [ancModeSelected]);

    return (
        <Paper elevation={0} sx={{  marginTop: 1, marginBottom: 1, display: "flex", minWidth: 275, justifyContent: "center", alignItems: "center" }}>
            <Grid sx={{ paddingLeft:0, justifyContent: "center" }}>
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
                    <Collapse in={sliderPosition == "left"} mountOnEnter unmountOnExit>
                        <ANCModeSelection setAncModeSelected={setAncModeSelected} />
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

function ANCModeSelection({ setAncModeSelected }: { setAncModeSelected: React.Dispatch<React.SetStateAction<ANCModes>> }) {

    let [modeSelected, setModeSelected] = useState<ANCModes | "AncCustomValue">("AncIndoorMode");
    let [customModeSelected, setCustomModeSelected] = useState<boolean>(false);
    let [customValue, setCustomValue] = useState<number | number[]>(10);

    useEffect(() => {
        // Indoor mode by default
        if (modeSelected == "AncCustomValue") {
            setCustomModeSelected(true)
        } else {
            setCustomModeSelected(false)
            setAncModeSelected(modeSelected);
        }


    }, [modeSelected]);


    useEffect(() => {
        if (customModeSelected) {
            setAncModeSelected({ AncCustomValue: customValue as number });
        }
    }, [customValue]);


    return (
        <Stack>
            <Grid container direction="row" spacing={1} sx={{ display: "flex", justifyContent: "space-evenly", pt: 2 }}>
                <Grid item><ANCModeButton variant="outlined" active={modeSelected == "AncTransportMode"} onClick={() => { setModeSelected("AncTransportMode") }} size="small">Transport</ANCModeButton></Grid>
                <Grid item><ANCModeButton variant="outlined" active={modeSelected == "AncOutdoorMode"} onClick={() => { setModeSelected("AncOutdoorMode") }} size="small">Outdoor</ANCModeButton></Grid>
                <Grid item><ANCModeButton variant="outlined" active={modeSelected == "AncIndoorMode"} onClick={() => { setModeSelected("AncIndoorMode") }} size="small">Indoor</ANCModeButton></Grid>
                <Grid item><ANCModeButton variant="outlined" active={modeSelected == "AncCustomValue"} onClick={() => { setModeSelected("AncCustomValue") }} size="small">Custom</ANCModeButton></Grid>

            </Grid>
            <Collapse in={customModeSelected}>
                <Slider
                    size="small"
                    defaultValue={10}
                    onChange={(_, newValue) => setCustomValue(newValue)}
                    onChangeCommitted={(_, newValue) => setCustomValue(newValue)}
                    sx={{ mt: 2, pb: 0, width: "98%" }}
                    min={0}
                    max={10}
                    marks
                    aria-label="Small"
                    valueLabelDisplay="auto"
                />
            </Collapse>
        </Stack>
    );
}