import { Button, Collapse, Grid, Icon, Paper, Slider, Stack, styled } from '@mui/material';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import ANCIcon from '@assets/ambient_icon_anc.png';
import NormalIcon from '@assets/ambient_icon_off.png';
import TransIcon from '@assets/ambient_icon_trans.png';
import React, { useCallback, useEffect, useState } from 'react';
import {
  BluetoothAdrr,
  CurrentSoundMode,
  SoundcoreDeviceState,
  SoundMode
} from '@generated-types/soundcore-lib';
import { useUpdateDeviceSoundMode } from '@hooks/useDeviceCommand';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import { BLEDevice } from '../../ble/bleDevice';

export interface SoundModeCardProps {
  state: SoundcoreDeviceState;
}

export const SoundModeCard = ({ state }: SoundModeCardProps): JSX.Element => {
  const {
    soundMode: soundModeState,
    featureSet: {
      soundModeFeatures: {
        allowedAncModes: ancFeatures,
        allowedTransparencyModes: transparencyFeatures,
        hasNormal: hasNormalMode,
        maxCustomAnc: maxCustomAncValue,
        maxCustomTransparency: maxCustomTransValue
      } = {}
    }
  } = state;

  const deviceAddrOrDevice: BluetoothAdrr | BLEDevice | null = window.isTauri
    ? useTauriManagerStore((state) => state.currentViewedDevice)
    : useWebManagerStore((state) => state.device);

  if (
    !soundModeState ||
    !deviceAddrOrDevice ||
    !ancFeatures ||
    !transparencyFeatures ||
    !hasNormalMode
  ) {
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

  const mapModeToCurrentSoundModeKey = useCallback((mode: CurrentSoundMode) => {
    const lowerCaseMode = mode.toLowerCase();
    if (lowerCaseMode === CurrentSoundMode.ANC.toLowerCase()) {
      return 'ancMode';
    } else if (lowerCaseMode === CurrentSoundMode.Transparency.toLowerCase()) {
      return 'transMode';
    }
    return null;
  }, []);

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

  const [selectedSoundMode, setSelectedSoundMode] = useState<SoundMode>(soundModeState);

  // Synchronize external changes originating from the device
  useEffect(() => {
    setSelectedSoundMode(soundModeState);
    setIcon(mapPositionToIcon(mapModeToPosition(soundModeState.current)));
  }, [soundModeState]);

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

  const handleCurrentSoundModeChange = (soundMode: CurrentSoundMode) => {
    useUpdateDeviceSoundMode(deviceAddrOrDevice, {
      ...selectedSoundMode,
      current: soundMode
    });
  };

  const handleCustomValueChange = (value: number) => {
    if (selectedSoundMode.current === CurrentSoundMode.ANC) {
      useUpdateDeviceSoundMode(deviceAddrOrDevice, {
        ...selectedSoundMode,
        customAnc: value
      });
    } else if (selectedSoundMode.current === CurrentSoundMode.Transparency) {
      useUpdateDeviceSoundMode(deviceAddrOrDevice, {
        ...selectedSoundMode,
        customTrans: value
      });
    }
  };

  const currentNonNormalSoundModeKey = mapModeToCurrentSoundModeKey(selectedSoundMode.current);
  let currentSubValue = '';
  let currentCustomAncOrTransValue = 0;
  let maxCustomSliderValue = 0;
  let currentSoundModeType: string;
  let isCustomSoundModeSelected: boolean = false;

  if (currentNonNormalSoundModeKey) {
    currentSubValue = selectedSoundMode[currentNonNormalSoundModeKey].value as string;
    currentSoundModeType = selectedSoundMode[currentNonNormalSoundModeKey].type;
    isCustomSoundModeSelected =
      selectedSoundMode[currentNonNormalSoundModeKey].value.toLowerCase() === 'custom';
    if (currentNonNormalSoundModeKey === 'ancMode') {
      currentCustomAncOrTransValue = selectedSoundMode.customAnc;
      maxCustomSliderValue = maxCustomAncValue || 5;
    } else if (currentNonNormalSoundModeKey === 'transMode') {
      currentCustomAncOrTransValue = selectedSoundMode.customTrans || 0;
      maxCustomSliderValue = maxCustomTransValue || 5;
    }
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
                setSliderPosition={() => handleCurrentSoundModeChange(CurrentSoundMode.ANC)}
              />
            )}
            {hasNormalMode && (
              <SliderButton
                position="center"
                icon={NormalIcon}
                setSliderIcon={setIcon}
                setSliderPosition={() => handleCurrentSoundModeChange(CurrentSoundMode.Normal)}
              />
            )}
            {transparencyFeatures.length > 0 && (
              <SliderButton
                position="right"
                icon={TransIcon}
                setSliderIcon={setIcon}
                setSliderPosition={() =>
                  handleCurrentSoundModeChange(CurrentSoundMode.Transparency)
                }
              />
            )}
          </SliderSelectorWrapper>
        </Grid>

        <Collapse in={modeButtons && modeButtons.length > 0} timeout="auto">
          <ModeGroupButtons
            buttons={modeButtons}
            onClick={(value) => {
              if (currentNonNormalSoundModeKey) {
                useUpdateDeviceSoundMode(deviceAddrOrDevice, {
                  ...selectedSoundMode,
                  [currentNonNormalSoundModeKey]: { type: currentSoundModeType, value }
                });
              }
            }}
            selectedValue={currentSubValue}
          />
        </Collapse>
        <Collapse in={isCustomSoundModeSelected} timeout="auto">
          <Grid item>
            <CustomValueSlider
              value={currentCustomAncOrTransValue}
              maxValue={maxCustomSliderValue}
              onChange={(v) => handleCustomValueChange(v)}
            />
          </Grid>
        </Collapse>
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
  backgroundColor: active ? theme.palette.primary.dark : 'transparent',
  color: active ? theme.palette.text.primary : theme.palette.text.secondary
}));

export type AllowedSliderPositions = 'left' | 'right' | 'center';

const SliderSelectorWrapper = styled('div')(({ theme }) => ({
  width: window.innerWidth - 35 - (window.innerWidth - 35) / 3,
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
  zIndex: 2,
  display: 'flex',
  flexDirection: 'row',
  position: 'absolute',
  backgroundColor: theme.palette.primary.dark,
  borderRadius: 28,
  height: 53,
  alignItems: 'center',
  justifyContent: 'center',
  width: (window.innerWidth - 35 - (window.innerWidth - 35) / 3) / 3,
  elevation: 4,
  shadowColor: 'black',
  shadowRadius: 10,
  shadowOpacity: 0.31,
  transition: 'transform 0.32s cubic-bezier(0.87, 0, 0.13, 1)',
  ...(position == 'left' && {
    transform: `translateX(-${(window.innerWidth - 35 - (window.innerWidth - 35) / 3) / 3}px)`
  }),
  ...(position == 'right' && {
    transform: `translateX(${(window.innerWidth - 35 - (window.innerWidth - 35) / 3) / 3}px)`
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
  width: (window.innerWidth - 35 - (window.innerWidth - 35) / 3) / 3,
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

const CustomValueSlider: React.FC<{
  value: number;
  maxValue: number;
  onChange: (value: number) => void;
}> = ({ value, maxValue, onChange }) => {
  return (
    <Slider
      size="small"
      value={value}
      onChange={(_e, v) => onChange(v as number)}
      min={0}
      max={maxValue}
      sx={{ mt: 2, pb: 0, width: '98%' }}
      marks
    />
  );
};
