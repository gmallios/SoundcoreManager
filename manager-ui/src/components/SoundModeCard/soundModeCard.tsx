import { Button, Collapse, Grid, Icon, Paper, Stack, styled } from '@mui/material';
import { useSoundcoreStore } from '@stores/useSoundcoreStore';
import ANCIcon from '../../assets/ambient_icon_anc.png';
import NormalIcon from '../../assets/ambient_icon_off.png';
import TransIcon from '../../assets/ambient_icon_trans.png';
import React, { useCallback, useEffect, useState } from 'react';
import { CurrentSoundMode, DeviceFeatureSet, SoundMode } from '@generated-types/soundcore-lib';
import { useUpdateDeviceSoundMode } from '@hooks/useDeviceCommand';

export const SoundModeCard: React.FC<{ features: DeviceFeatureSet }> = ({ features }) => {
  if (!features.soundModeFeatures) {
    return <></>;
  }

  const ancFeatures = features.soundModeFeatures.allowedAncModes;
  const transparencyFeatures = features.soundModeFeatures.allowedTransparencyModes;
  const hasNormalMode = features.soundModeFeatures.hasNormalMode;
  const deviceSoundModeState = useSoundcoreStore((state) =>
    state.currentViewedDeviceState()
  )?.soundMode;
  const deviceAddr = useSoundcoreStore((state) => state.currentViewedDevice);

  if (!deviceSoundModeState || !deviceAddr) {
    return <></>;
  }

  const mapModeToPosition = useCallback((mode: CurrentSoundMode) => {
    const lowerCaseMode = mode.toLowerCase();
    if (lowerCaseMode === CurrentSoundMode.ANC.toLowerCase()) {
      return 'left';
    } else if (lowerCaseMode === CurrentSoundMode.Transparency.toLowerCase()) {
      return 'right';
    } else {
      return 'center';
    }
  }, []);

  const mapModeToCurrentSoundModeKey = useCallback(
    (mode: CurrentSoundMode): keyof SoundMode | null => {
      const lowerCaseMode = mode.toLowerCase();
      if (lowerCaseMode === CurrentSoundMode.ANC.toLowerCase()) {
        return 'ancMode';
      } else if (lowerCaseMode === CurrentSoundMode.Transparency.toLowerCase()) {
        return 'transMode';
      }
      return null;
    },
    []
  );

  const mapModeToFeatures = useCallback((mode: CurrentSoundMode) => {
    const lowerCaseMode = mode.toLowerCase();
    if (lowerCaseMode === CurrentSoundMode.ANC.toLowerCase()) {
      return ancFeatures;
    } else if (lowerCaseMode === CurrentSoundMode.Transparency.toLowerCase()) {
      return transparencyFeatures;
    }
    return [];
  }, []);

  const mapPositionToIcon = useCallback((position: AllowedSliderPositions) => {
    if (position === 'left') {
      return ANCIcon;
    } else if (position === 'right') {
      return TransIcon;
    } else {
      return NormalIcon;
    }
  }, []);

  const [selectedSoundMode, setSelectedSoundMode] = useState<SoundMode>(deviceSoundModeState);

  // Synchronize external changes originating from the device
  useEffect(() => {
    setSelectedSoundMode(deviceSoundModeState);
    setIcon(mapPositionToIcon(mapModeToPosition(deviceSoundModeState.current)));
  }, [deviceSoundModeState]);

  useEffect(() => {
    console.log('selectedSoundMode', selectedSoundMode);
  }, [selectedSoundMode]);

  const [icon, setIcon] = useState<string>(
    mapPositionToIcon(mapModeToPosition(selectedSoundMode.current))
  );
  const position = mapModeToPosition(selectedSoundMode.current);
  const modeButtons = mapModeToFeatures(selectedSoundMode.current)
    .map((mode) => {
      return { title: mode.value, value: mode.value };
    })
    .sort((a, b) => {
      if (a && a.title && b && b.title) {
        return a.title.localeCompare(b.title);
      }
      return 0;
    });

  const currentSoundModeKey = mapModeToCurrentSoundModeKey(selectedSoundMode.current);
  let currentSubValue;
  let currentSoundModeType: string;
  let hasCustomValueSlider: boolean = false;

  if (currentSoundModeKey) {
    currentSubValue = selectedSoundMode[currentSoundModeKey].value;
    currentSoundModeType = selectedSoundMode[currentSoundModeKey].type;
    hasCustomValueSlider = currentSoundModeType.toLowerCase() === 'custom';
  }

  return (
    <Paper
      elevation={0}
      sx={{
        marginTop: 0,
        marginBottom: 1,
        display: 'flex',
        minWidth: 275,
        justifyContent: 'center',
        alignItems: 'center'
      }}>
      <Grid sx={{ paddingLeft: 0, justifyContent: 'center' }}>
        <Grid item>
          <SliderSelectorWrapper>
            <SliderSelector position={position}>
              <Icon sx={{ display: 'flex', width: 32, height: 32, zIndex: 0 }}>
                <img src={icon} height="32" />
              </Icon>
            </SliderSelector>
            {ancFeatures.length > 0 && (
              <SliderButton
                position="left"
                icon={ANCIcon}
                setSliderIcon={setIcon}
                setSliderPosition={() =>
                  useUpdateDeviceSoundMode(deviceAddr, {
                    ...selectedSoundMode,
                    current: CurrentSoundMode.ANC
                  })
                }
              />
            )}
            {hasNormalMode && (
              <SliderButton
                position="center"
                icon={NormalIcon}
                setSliderIcon={setIcon}
                setSliderPosition={() =>
                  useUpdateDeviceSoundMode(deviceAddr, {
                    ...selectedSoundMode,
                    current: CurrentSoundMode.Normal
                  })
                }
              />
            )}
            {transparencyFeatures.length > 0 && (
              <SliderButton
                position="right"
                icon={TransIcon}
                setSliderIcon={setIcon}
                setSliderPosition={() =>
                  useUpdateDeviceSoundMode(deviceAddr, {
                    ...selectedSoundMode,
                    current: CurrentSoundMode.Transparency
                  })
                }
              />
            )}
          </SliderSelectorWrapper>
        </Grid>

        <Collapse in={modeButtons && modeButtons.length > 0} timeout="auto">
          <ModeGroupButtons
            buttons={modeButtons}
            onClick={(value) => {
              if (currentSoundModeKey) {
                useUpdateDeviceSoundMode(deviceAddr, {
                  ...selectedSoundMode,
                  [currentSoundModeKey]: { type: currentSoundModeType, value }
                });
              }
            }}
            selectedValue={currentSubValue}
          />
        </Collapse>
        {hasCustomValueSlider && <h1> Custom</h1>}
        {/* <SliderSubButtons layout={layout} position={position} /> */}
      </Grid>
    </Paper>
  );
};

const ModeGroupButtons: React.FC<{
  buttons: Array<{ title: string; value: string }>;
  selectedValue: string;
  onClick: (value: string) => void;
}> = ({ buttons, selectedValue, onClick }) => {
  return (
    <Grid item sx={{ paddingTop: '0px !important' }}>
      <Stack>
        <Grid
          container
          direction="row"
          spacing={1}
          sx={{ display: 'flex', justifyContent: 'space-evenly', pt: 2 }}>
          {buttons.map((button) => (
            <ModeGroupButton
              key={button.value}
              active={selectedValue === button.value}
              onClick={() => onClick(button.value)}>
              {button.title}
            </ModeGroupButton>
          ))}
        </Grid>
      </Stack>
    </Grid>
  );
};

const ModeGroupButton = styled(Button, {
  shouldForwardProp: (prop) => prop !== 'active'
})<{ active?: boolean }>(({ theme, active }) => ({
  //width: "100px",
  backgroundColor: active ? theme.palette.primary.dark : 'transparent',
  color: active ? theme.palette.text.primary : theme.palette.text.secondary
}));

const width = window.innerWidth - 35;

const Metrics = {
  containerWidth: width - 30,
  switchWidth: width / 2.7
};

export type AllowedSliderPositions = 'left' | 'right' | 'center';

const SliderSelectorWrapper = styled('div')(({ theme }) => ({
  width: Metrics.containerWidth,
  height: 55,
  display: 'flex',
  flexDirection: 'row',
  backgroundColor: theme.palette.background.paper,
  alignItems: 'center',
  justifyContent: 'center',
  borderWidth: 1,
  borderColor: theme.palette.divider,
  borderStyle: 'solid',
  borderRadius: 27.5
}));
const SliderSelector = styled('div', {
  shouldForwardProp: (prop) => prop !== 'position'
})<{ position: AllowedSliderPositions }>(({ position, theme }) => ({
  /* Remove border radius for miiddle item and animate it */
  zIndex: 2,
  display: 'flex',
  flexDirection: 'row',
  position: 'absolute',
  backgroundColor: theme.palette.primary.dark,
  borderRadius: 28,
  height: 53,
  alignItems: 'center',
  justifyContent: 'center',
  width: Metrics.switchWidth,
  elevation: 4,
  shadowColor: 'black',
  shadowRadius: 10,
  shadowOpacity: 0.31,
  transition: 'transform 0.32s cubic-bezier(0.87, 0, 0.13, 1)',
  ...(position == 'left' && {
    transform: 'translateX(-78%)'
  }),
  ...(position == 'right' && {
    transform: 'translateX(78%)'
  }),
  ...(position == 'center' && {
    transform: 'translateX(0)'
  })
}));

const SliderButtonInner = styled(Button, {
  shouldForwardProp: (prop) => prop !== 'position'
})<{ position: AllowedSliderPositions }>(({ position }) => ({
  display: 'flex',
  flex: 1,
  width: Metrics.containerWidth / 3,
  height: 54,
  justifyContent: 'center',
  alignItems: 'center',
  ...(position == 'left' && {
    borderRadius: '28px 0 0 28px'
  }),
  ...(position == 'center' && {
    borderRadius: '0 0 0 0'
  }),
  ...(position == 'right' && {
    borderRadius: '0 28px 28px 0'
  })
}));

const SliderButton: React.FC<{
  position: AllowedSliderPositions;
  icon: string;
  setSliderIcon: React.Dispatch<React.SetStateAction<string>>;
  setSliderPosition?: React.Dispatch<React.SetStateAction<AllowedSliderPositions>>;
}> = ({ position, icon, setSliderIcon, setSliderPosition }) => {
  return (
    <SliderButtonInner
      position={position}
      variant="text"
      onClick={() => {
        setSliderPosition && setSliderPosition(position);
        setSliderIcon(icon);
      }}>
      <Icon sx={{ display: 'flex', width: 32, height: 32, zIndex: 0 }}>
        <img src={icon} height="32" />
      </Icon>
    </SliderButtonInner>
  );
};
