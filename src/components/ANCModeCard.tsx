import { Button, Collapse, Grid, Icon, Paper, Slider, Stack, styled } from "@mui/material";
import { Children, ReactNode, useEffect, useState } from "react";
import ANCIcon from "../assets/ambient_icon_anc.png";
import NormalIcon from "../assets/ambient_icon_off.png";
import TransIcon from "../assets/ambient_icon_trans.png";
import useDeviceStore from "../hooks/useDeviceStore";
import { ANCModes } from "../bindings/ANCModes";
import { useUpdateANC } from "../hooks/useSoundcoreDevice";

const width = window.innerWidth - 35;

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

export type AllowedSliderPositions = "left" | "right" | "center" | null;

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
    transition: "transform 0.32s cubic-bezier(0.87, 0, 0.13, 1)",
    ...(position == "left" && {
        transform: "translateX(-78%)",
    }),
    ...(position == "right" && {
        transform: "translateX(78%)",
    }),
    ...(position == "center" && {
        transform: "translateX(0)",
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

    const { currentANCMode } = useDeviceStore();

    let [sliderPosition, setSliderPosition] = useState<AllowedSliderPositions>(null);
    let [sliderIcon, setSliderIcon] = useState<string>(NormalIcon);
    let [submenuOpen, setSubmenuOpen] = useState<boolean>(false);
    let [ancModeSelected, setAncModeSelected] = useState<ANCModes | "AncCustomValue">("AncOutdoorMode");
    let [transModeSelected, setTransModeSelected] = useState<ANCModes>("TransparencyFullyTransparentMode");
    let [ancCustomValue, setAncCustomValue] = useState<number | number[] | null>(null);
    let ancMutation = useUpdateANC();


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
        ];

    useEffect(() => {
        if (currentANCMode != null) {
            if (currentANCMode == "NormalMode") {
                setSliderPosition("center");
            } else if (currentANCMode == "TransparencyFullyTransparentMode" || currentANCMode == "TransparencyVocalMode") {
                setSliderPosition("right");
                setTransModeSelected(currentANCMode);
            } else {
                setSliderPosition("left");
                if (typeof currentANCMode === "object") {
                    setAncCustomValue(currentANCMode.AncCustomValue);
                    setAncModeSelected("AncCustomValue");
                } else {
                    setAncModeSelected(currentANCMode);
                }
            }
        }
    }, []);


    // TODO: Load persisted values
    useEffect(() => {
        if (sliderPosition == "center") {
            ancMutation.mutate("NormalMode");
            setSubmenuOpen(false);
        } else if (sliderPosition == "left") {
            if(ancModeSelected != "AncCustomValue") {
                ancMutation.mutate(ancModeSelected);
            }
            setSubmenuOpen(true);
        } else if (sliderPosition == "right") {
            ancMutation.mutate(transModeSelected);
            setSubmenuOpen(true);
        }
    }, [sliderPosition]);

    useEffect(() => {
        if (sliderPosition == "left") {
            if (ancModeSelected == "AncCustomValue") {
                if(ancCustomValue == null) {
                    setAncCustomValue(10);
                }
                ancMutation.mutate({ AncCustomValue: ancCustomValue as number });
            } else {
                ancMutation.mutate(ancModeSelected);
            }
        }
    }, [ancModeSelected, ancCustomValue]);

    useEffect(() => {
        if (sliderPosition == "right") {
            ancMutation.mutate(transModeSelected);
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
                    <Collapse in={submenuOpen}>
                        <ButtonGrid buttonArray={sliderPosition == "left" ? ancButtons : transButtons} setButtonSelected={sliderPosition == "left" ? setAncModeSelected : setTransModeSelected} buttonSelected={sliderPosition == "left" ? ancModeSelected : transModeSelected}>
                            <Collapse in={sliderPosition == "left" && ancModeSelected == "AncCustomValue"}>
                                {ancCustomValue != null &&
                                    <Slider
                                        size="small"
                                        value={ancCustomValue}
                                        onChange={(_, newValue) => setAncCustomValue(newValue)}
                                        onChangeCommitted={(_, newValue) => setAncCustomValue(newValue)}
                                        sx={{ mt: 2, pb: 0, width: "98%" }}
                                        min={0}
                                        max={10}
                                        marks
                                        aria-label="Small"
                                        valueLabelDisplay="auto"
                                    />}
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