import { BluetoothAdrr, EQProfile, MonoEQ, SoundMode } from '@generated-types/soundcore-lib';
import { useAsyncBridgeRequest } from './useAsyncBridge';

// TODO: Implement using promise, waiting for the Ack response from the async bridge
// We can generate a UUID which comes back to track the flow
export const useUpdateDeviceSoundMode = (addr: BluetoothAdrr, mode: SoundMode) => {
  useAsyncBridgeRequest({
    command: 'setSoundMode',
    payload: {
      addr: addr,
      payload: mode
    }
  });
};

export const useUpdatePresetEqualizer = (addr: BluetoothAdrr, preset: EQProfile) => {
  useAsyncBridgeRequest({
    command: 'setEqualizer',
    payload: {
      addr,
      payload: {
        command: 'setEqualizerPreset',
        payload: preset
      }
    }
  });
};

export const useUpdateCustomEqualizer = (addr: BluetoothAdrr, eq: MonoEQ) => {
  useAsyncBridgeRequest({
    command: 'setEqualizer',
    payload: {
      addr,
      payload: {
        command: 'setCustomEqualizer',
        payload: eq
      }
    }
  });
};
