import {
  BluetoothAdrr,
  CurrentSoundMode,
  SoundcoreDeviceState,
  SoundMode
} from '@generated-types/soundcore-lib';
import React, { useCallback, useEffect, useState } from 'react';
import { Image, Slider, Tab, Tabs } from '@nextui-org/react';
import { BLEDevice } from '../../ble/bleDevice';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import ANCIcon from '@assets/ambient_icon_anc.png';
import TransIcon from '@assets/ambient_icon_trans.png';
import NormalIcon from '@assets/ambient_icon_off.png';
import { useUpdateDeviceSoundMode } from '@hooks/useDeviceCommand';

export interface SoundModeTabsProps {
  state: SoundcoreDeviceState;
}

export const SoundModeTabs: React.FC<SoundModeTabsProps> = ({ state }: SoundModeTabsProps) => {
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

  const [selectedSoundMode, setSelectedSoundMode] = useState<SoundMode>(soundModeState);

  // Synchronize external changes originating from the device
  useEffect(() => {
    setSelectedSoundMode(soundModeState);
  }, [soundModeState]);

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

  const handleCustomValueChange = (value: number | number[]) => {
    if (Array.isArray(value)) return;
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
  let isCustomSoundModeSelected: boolean = false;

  if (currentNonNormalSoundModeKey) {
    currentSubValue = selectedSoundMode[currentNonNormalSoundModeKey].value as string;
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

  const handleSubSoundModeChange = (subMode: string) => {
    if (selectedSoundMode.current === CurrentSoundMode.Normal) return;

    const soundModeKey =
      selectedSoundMode.current === CurrentSoundMode.ANC ? 'ancMode' : 'transMode';
    useUpdateDeviceSoundMode(deviceAddrOrDevice, {
      ...selectedSoundMode,
      [soundModeKey]: {
        type: selectedSoundMode[soundModeKey].type,
        value: subMode
      }
    });
  };

  return (
    <div className={'flex flex-col w-full justify-center items-center gap-2'}>
      <CurrentSoundModeTabs
        selectedSoundMode={selectedSoundMode}
        onChange={handleCurrentSoundModeChange}
      />
      <SubSoundModeTabs
        buttons={modeButtons}
        selectedValue={currentSubValue}
        onClick={handleSubSoundModeChange}
      />
      <Slider
        className={`${isCustomSoundModeSelected ? 'visible transition-all ease-in-out' : 'invisible'} transition-opacity ease-in-out`}
        size={'sm'}
        step={1}
        minValue={0}
        showSteps={true}
        value={currentCustomAncOrTransValue}
        onChange={handleCustomValueChange}
        maxValue={maxCustomSliderValue}
      />
    </div>
  );
};

interface CurrentSoundModeTabsProps {
  selectedSoundMode: SoundMode;
  onChange: (currentMode: CurrentSoundMode) => void;
}

const CurrentSoundModeTabs: React.FC<CurrentSoundModeTabsProps> = ({
  selectedSoundMode,
  onChange
}) => {
  const imageClassName = 'max-w-full max-h-full h-7 object-contain';
  return (
    <Tabs
      color={'primary'}
      variant={'bordered'}
      className={'w-full pt-2 flex'}
      classNames={{ tabList: 'w-full' }}
      size={'md'}
      selectedKey={selectedSoundMode.current}
      onSelectionChange={(k) => onChange(k as CurrentSoundMode)}>
      <Tab
        key={CurrentSoundMode.ANC}
        title={
          <div className="flex items-center space-x-2">
            <span>ANC</span>
            <Image src={ANCIcon} className={imageClassName} />
          </div>
        }
      />
      <Tab
        key={CurrentSoundMode.Normal}
        title={
          <div className="flex items-center space-x-2">
            <span>Normal</span>
            <Image src={NormalIcon} className={imageClassName} />
          </div>
        }
      />
      <Tab
        key={CurrentSoundMode.Transparency}
        title={
          <div className="flex items-center space-x-2">
            <span>Transparency</span>
            <Image src={TransIcon} className={imageClassName} />
          </div>
        }
      />
    </Tabs>
  );
};

interface SubSoundModeTabsProps {
  buttons: Array<{ title: string; value: string }>;
  selectedValue: string;
  onClick: (value: string) => void;
}

const SubSoundModeTabs: React.FC<SubSoundModeTabsProps> = ({ buttons, selectedValue, onClick }) => {
  return (
    <Tabs
      selectedKey={selectedValue}
      onSelectionChange={(k) => onClick(k as string)}
      variant={'underlined'}>
      {buttons.map((b) => (
        <Tab key={b.value} title={b.title} />
      ))}
    </Tabs>
  );
};
