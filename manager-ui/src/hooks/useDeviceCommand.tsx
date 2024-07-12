import { BluetoothAdrr, EQProfile, SoundMode } from '@generated-types/soundcore-lib';
import { useAsyncBridgeRequest } from './useAsyncBridge';
import { BLEDevice } from '../ble/bleDevice';

// TODO: Implement using promise, waiting for the Ack response from the async bridge
// We can generate a UUID which comes back to track the flow
export const useUpdateDeviceSoundMode = async (ref: BluetoothAdrr | BLEDevice, mode: SoundMode) => {
  if (window.isTauri && 'address' in ref) {
    return useAsyncBridgeRequest({
      command: 'setSoundMode',
      payload: {
        addr: ref,
        payload: mode
      }
    });
  } else if (ref instanceof BLEDevice) {
    return ref.setSoundMode(mode);
  }
};

export const useUpdatePresetEqualizer = async (ref: BluetoothAdrr | BLEDevice, preset: EQProfile) => {
  if (window.isTauri && 'address' in ref) {
    return useAsyncBridgeRequest({
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
    return ref.setEqualizerPreset(preset);
  }
};

/**
 * Set custom EQ values
 * @param values The values should be in range -60..=60
 */
export const useUpdateCustomEqualizer = async (ref: BluetoothAdrr | BLEDevice, values: number[]) => {
  if (window.isTauri) {
    return useAsyncBridgeRequest({
      command: 'setEqualizer',
      payload: {
        addr: ref,
        payload: {
          command: 'setCustomEqualizer',
          payload: values
        }
      }
    });
  } else if (ref instanceof BLEDevice) {
    return ref.setEqualizerCustom(values);
  }
};
