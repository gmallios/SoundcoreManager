import { BluetoothAdrr, EQProfile, SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import React, { useRef, useState } from 'react';
import { useUpdateCustomEqualizer, useUpdatePresetEqualizer } from '@hooks/useDeviceCommand';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import { BLEDevice } from '../../ble/bleDevice';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import { Button, Card, CardBody, CardFooter, Select, SelectItem, Switch } from '@nextui-org/react';
import { Equalizer, EqualizerRef } from '@components/EqualizerCard/equalizer';
import { getPresetEqValue } from '@wasm/manager_wasm';

export interface EqualizerCardProps {
  state: SoundcoreDeviceState;
}

export const EqualizerCard: React.FC<EqualizerCardProps> = ({ state }) => {
  const deviceAddrOrDevice: BluetoothAdrr | BLEDevice | null = window.isTauri
    ? useTauriManagerStore((state) => state.currentViewedDevice)
    : useWebManagerStore((state) => state.device);

  if (!deviceAddrOrDevice) {
    return <></>;
  }

  const isOnCustom = state.eqConfiguration.value.profile === EQProfile.Custom;
  const hasBassUp = state.featureSet.equalizerFeatures?.has_bass_up ?? false;

  const [presetEqValues, setPresetEqValues] = useState<number[]>([]);
  const eqRef = useRef<EqualizerRef>(null);

  const onCustomEqualizerChange = (output: number[]) => {
    if (isOnCustom) {
      const new_eq = output.map((v) => v * 10);
      useUpdateCustomEqualizer(deviceAddrOrDevice, new_eq);
    }
  };

  const eqProfileChange = (profile: EQProfile) => {
    setPresetEqValues(
      mapRangeArray(Array.from(getPresetEqValue(profile.toString(), 8)), 0, 240, -6, 6).map(
        (v) => v * 2
      )
    );
    useUpdatePresetEqualizer(deviceAddrOrDevice, profile);
  };

  const mapRange = (
    value: number,
    inMin: number,
    inMax: number,
    outMin: number,
    outMax: number
  ): number => {
    return ((value - inMin) * (outMax - outMin)) / (inMax - inMin) + outMin;
  };

  const mapRangeArray = (
    input: number[],
    inMin: number,
    inMax: number,
    outMin: number,
    outMax: number
  ): number[] => {
    return input.map((value) => mapRange(value, inMin, inMax, outMin, outMax));
  };

  const getMappedCustomEqValues = (): number[] => {
    let valueArr;
    if (state.eqConfiguration.value.eq && 'left' in state.eqConfiguration.value.eq) {
      valueArr = state.eqConfiguration.value.eq.left.values;
    } else {
      valueArr = state.eqConfiguration.value.eq.values;
    }
    return mapRangeArray(valueArr, 0, 240, -6, 6).map((v) => v * 2);
  };

  const eqProfiles = Object.keys(EQProfile).filter((item) => {
    return isNaN(Number(item)) && item !== 'Custom' && (!hasBassUp || item !== 'BassBooster');
  });

  const onCardPress = (v: string) => {
    const newProfile = v == 'Custom' ? EQProfile.Custom : EQProfile.SoundcoreSignature;
    if (!isOnCustom && newProfile !== 'Custom') {
      return;
    }
    useUpdatePresetEqualizer(deviceAddrOrDevice, newProfile);
  };

  const onBassUpChange = (v: boolean) => {
    useUpdatePresetEqualizer(
      deviceAddrOrDevice,
      v ? EQProfile.BassBooster : EQProfile.SoundcoreSignature
    );
  };

  return (
    <Card shadow={'sm'} className={'m-5'}>
      <CardBody>
        {state.featureSet.equalizerFeatures && (
          <>
            <div className={'grid grid-cols-2 grid-rows-1 gap-5'}>
              <EQModeCard
                title={'Preset'}
                isSelected={!isOnCustom}
                hasBassUp={hasBassUp}
                profiles={eqProfiles}
                currentEqProfile={state.eqConfiguration.value.profile}
                bassUpValue={state.eqConfiguration.value.profile === EQProfile.BassBooster}
                onPress={onCardPress}
                onPresetChange={eqProfileChange}
                onBassUpChange={onBassUpChange}
              />
              <EQModeCard
                showResetEq
                title={'Custom'}
                isSelected={isOnCustom}
                onPress={onCardPress}
                onResetEq={eqRef.current?.onReset}
              />
            </div>
            <div className={'w-full'}>
              <Equalizer
                bands={state.featureSet.equalizerFeatures.bands}
                input={isOnCustom ? [...getMappedCustomEqValues()] : [...presetEqValues]}
                onEqualizerChange={onCustomEqualizerChange}
                ref={eqRef}
                disabled={!isOnCustom}
              />
            </div>
          </>
        )}
      </CardBody>
    </Card>
  );
};

interface EQModeCardProps {
  title: 'Preset' | 'Custom';
  isSelected: boolean;
  hasBassUp?: boolean;
  currentEqProfile?: EQProfile;
  profiles?: Array<string>;
  bassUpValue?: boolean;
  onBassUpChange?: (v: boolean) => void;
  onPress?: (e: 'Preset' | 'Custom') => void;
  showResetEq?: boolean;
  onResetEq?: () => void;
  onPresetChange?: (preset: EQProfile) => void;
}

const EQModeCard: React.FC<EQModeCardProps> = ({
  title,
  isSelected,
  hasBassUp,
  currentEqProfile,
  profiles,
  bassUpValue,
  onBassUpChange,
  onPress,
  showResetEq,
  onResetEq,
  onPresetChange
}) => {
  const visibleEqProfile = !bassUpValue
    ? (currentEqProfile as string)
    : EQProfile.SoundcoreSignature;

  return (
    <Card
      isFooterBlurred
      isPressable
      radius="lg"
      className={
        'border-none col-span-1 h-24 hover:-translate-y-0.5 ease-in-out transition-all transform-gpu'
      }
      style={{
        ...(isSelected && {
          outlineColor: 'hsl(var(--nextui-focus) / var(--nextui-focus-opacity, 1))',
          outlineOffset: '2px',
          outlineWidth: '2px'
        })
      }}
      onPress={() => onPress && onPress(title)}>
      <div
        className={'bg-default-100 p-3 pb-0'}
        style={{
          width: '100%',
          height: '100%'
        }}>
        <div className={'flex justify-between'}>
          <div className={'flex flex-col items-start gap-1'}>
            <p className={'text-white'}>{title}</p>
            {profiles && profiles.length > 0 && bassUpValue !== undefined && hasBassUp && (
              <div className={'flex flex-row items-center gap-1'}>
                <p className="text-small text-default-600 h-fit">BassUp</p>
                <Switch
                  isSelected={bassUpValue}
                  onValueChange={onBassUpChange}
                  size="sm"
                  disabled={!isSelected}
                />
              </div>
            )}
          </div>
        </div>
      </div>
      <CardFooter className={'p-0 h-10'}>
        {profiles && profiles.length > 0 && (
          <Select
            //TODO: Remove hack and fix the styles
            label={!profiles.includes(visibleEqProfile) ? 'Select a profile' : ''}
            className="w-full p-0"
            size="md"
            isDisabled={!isSelected}
            onSelectionChange={(e) => {
              onPresetChange && onPresetChange([...e][0] as EQProfile);
            }}
            selectedKeys={[visibleEqProfile]}>
            {profiles.map((p) => (
              <SelectItem key={p}>{p}</SelectItem>
            ))}
          </Select>
        )}
        {showResetEq && (
          <Button
            className={'w-full'}
            disabled={!isSelected}
            onClick={() => {
              onResetEq && onResetEq();
            }}>
            Reset
          </Button>
        )}
      </CardFooter>
      {/*<CardFooter className="justify-center before:bg-white/10 border-white/20 border-1 overflow-hidden py-1 absolute before:rounded-xl rounded-large bottom-1 w-[calc(100%_-_8px)] shadow-small ml-1 z-10">*/}
      {/*  <p className="text-tiny">{title}</p>*/}
      {/*</CardFooter>*/}
    </Card>
  );
};
