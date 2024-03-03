import { StateCreator } from 'zustand';
import { DiscoveredDevice } from '../types/soundcore-lib';

export const createBluetoothSlice: StateCreator<BluetoothSlice> = (set, _get) => ({
  latestScan: null,
  setLatestScan: (scanRes: DiscoveredDevice[]) => set({ latestScan: scanRes })
});

export interface BluetoothSlice {
  latestScan: DiscoveredDevice[] | null;
  setLatestScan: (scanRes: DiscoveredDevice[]) => void;
}
