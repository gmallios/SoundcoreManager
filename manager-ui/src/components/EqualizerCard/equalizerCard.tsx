import { Collapse, MenuItem, Paper, Select, SelectChangeEvent, Stack } from '@mui/material';
import { Equalizer } from './equalizer';
import { EQProfile, MonoEQ, SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import { useCallback } from 'react';
import { useUpdateCustomEqualizer, useUpdatePresetEqualizer } from '@hooks/useDeviceCommand';
import { useSoundcoreStore } from '@stores/useSoundcoreStore';

export interface EqualizerCardProps {
  state: SoundcoreDeviceState;
}

export const EqualizerCard = ({ state }: EqualizerCardProps): JSX.Element => {
  const deviceAddr = useSoundcoreStore((state) => state.currentViewedDevice);
  const isOnCustom = state.eqConfiguration.value.profile === EQProfile.Custom;
  const hasBassUp = state.featureSet.equalizerFeatures?.has_bass_up ?? false;

  const onCustomEqualizerChange = useCallback((output: number[]) => {
    if (isOnCustom) {
      const new_eq: MonoEQ = { values: mapRangeArray(output, -6, 6, 0, 240) };
      useUpdateCustomEqualizer(deviceAddr!, new_eq);
    }
  }, []);

  const onSelectedEqProfileChange = (e: SelectChangeEvent) => {
    console.log('Selected EQ profile:', e.target.value);
    useUpdatePresetEqualizer(deviceAddr!, e.target.value as EQProfile);
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

  const getMappedEqValues = (): number[] => {
    let valueArr;
    if (state.eqConfiguration.value.eq && 'left' in state.eqConfiguration.value.eq) {
      valueArr = state.eqConfiguration.value.eq.left.values;
    } else {
      valueArr = state.eqConfiguration.value.eq.values;
    }
    return mapRangeArray(valueArr, 0, 240, -6, 6);
  };

  const eqProfiles = Object.keys(EQProfile).filter((item) => {
    return isNaN(Number(item));
  });

  return (
    <Paper sx={{ display: 'flex', margin: 3, justifyContent: 'center', alignItems: 'center' }}>
      <Stack sx={{ width: '100%' }}>
        <Select value={state.eqConfiguration.value.profile} onChange={onSelectedEqProfileChange}>
          {eqProfiles
            .filter((prof) => {
              if (hasBassUp) {
                prof.toLowerCase() !== 'bassbooster';
              }
            })
            .map((profile) => (
              <MenuItem key={profile} value={profile}>
                {profile}
              </MenuItem>
            ))}
        </Select>
        {state.featureSet.equalizerFeatures && (
          <Collapse in={isOnCustom} timeout="auto">
            <Equalizer
              bands={state.featureSet.equalizerFeatures.bands}
              input={[...getMappedEqValues()]}
              onEqualizerChange={onCustomEqualizerChange}
            />
          </Collapse>
        )}
      </Stack>
    </Paper>
  );
};
