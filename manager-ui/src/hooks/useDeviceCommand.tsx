import { BluetoothAdrr, EQProfile, MonoEQ, SoundMode } from '@generated-types/soundcore-lib';
import { useAsyncBridgeRequest } from './useAsyncBridge';
import { BLEDevice } from '../ble/bleDevice';

// TODO: Implement using promise, waiting for the Ack response from the async bridge
// We can generate a UUID which comes back to track the flow
export const useUpdateDeviceSoundMode = (ref: BluetoothAdrr | BLEDevice, mode: SoundMode) => {
  if (window.isTauri && 'address' in ref) {
    useAsyncBridgeRequest({
      command: 'setSoundMode',
      payload: {
        addr: ref,
        payload: mode
      }
    });
  } else if (ref instanceof BLEDevice) {
  }
};

export const useUpdatePresetEqualizer = (ref: BluetoothAdrr | BLEDevice, preset: EQProfile) => {
  if (window.isTauri && 'address' in ref) {
    useAsyncBridgeRequest({
      command: 'setEqualizer',
      payload: {
        addr: ref,
        payload: {
          command: 'setEqualizerPreset',
          payload: preset
        }
      }
    });
  } else if (ref instanceof BLEDevice) {
  }
};

export const useUpdateCustomEqualizer = (ref: BluetoothAdrr | BLEDevice, eq: MonoEQ) => {
  if (window.isTauri) {
    useAsyncBridgeRequest({
      command: 'setEqualizer',
      payload: {
        addr: ref,
        payload: {
          command: 'setCustomEqualizer',
          payload: eq
        }
      }
    });
  } else if (ref instanceof BLEDevice) {
    const a = ref;
  }
};
