import { create } from 'zustand';
import { BLEDevice } from '../../ble/bleDevice';
import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';

export interface WebManagerStore {
  device: BLEDevice | null;
  currentState: SoundcoreDeviceState | null;
  setDevice: (device: BLEDevice | null) => void;
  setCurrentState: (currentState: SoundcoreDeviceState | null) => void;
}

export const useWebManagerStore = create<WebManagerStore>((set) => ({
  device: null,
  currentState: null,
  setDevice: (device) => set({ device }),
  setCurrentState: (currentState) => set({ currentState })
}));