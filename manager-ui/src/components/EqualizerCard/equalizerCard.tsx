import { Paper } from '@mui/material';
import { Equalizer } from './equalizer';
import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import { useCallback } from 'react';

export interface EqualizerCardProps {
  state: SoundcoreDeviceState;
}

export const EqualizerCard = ({ state }: EqualizerCardProps): JSX.Element => {
  const onEqualizerChange = useCallback((output: number[]) => {
    console.log('Equalizer output:', output);
    console.log('Equalizer output mapped:', mapRangeArray(output, -6, 6, 0, 240));
  }, []);

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
    console.log('EQ values:', valueArr);
    return mapRangeArray(valueArr, 0, 240, -6, 6);
  };

  console.log('Mapped EQ values:', getMappedEqValues());

  return (
    <Paper sx={{ display: 'flex', margin: 3, justifyContent: 'center', alignItems: 'center' }}>
      {state.featureSet.equalizerFeatures && (
        <Equalizer
          bands={state.featureSet.equalizerFeatures.bands}
          input={[...getMappedEqValues()]}
          onEqualizerChange={onEqualizerChange}
        />
      )}
    </Paper>
  );
};
