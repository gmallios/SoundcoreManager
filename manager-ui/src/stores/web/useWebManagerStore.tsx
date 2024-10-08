import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { BLEDevice } from '../../ble/bleDevice';
import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';

export interface WebManagerStore {
  device: BLEDevice | null;
  currentState: SoundcoreDeviceState | null;
  setDevice: (device: BLEDevice | null) => void;
  setCurrentState: (currentState: SoundcoreDeviceState | null) => void;
  disconnect: () => void;
}

export const useWebManagerStore = create<WebManagerStore>()(
  devtools((set) => ({
    device: null,
    currentState: null,
    setDevice: (device) => set({ device }),
    setCurrentState: (currentState) => set({ currentState }),
    disconnect: () =>
      set((state) => {
        state.device?.free();
        return { device: null, currentState: null };
      })
  }))
);
