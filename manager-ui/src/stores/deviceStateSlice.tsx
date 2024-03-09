import { StateCreator } from 'zustand';
import { BluetoothAdrr, SoundcoreDeviceState } from '../types/soundcore-lib';
import { NewStateResponse } from '../types/tauri-backend';

export const createDeviceStateSlice: StateCreator<DeviceStateSlice> = (set, get) => ({
  states: new Map(),
  setStateFromBridgeResponse: (resp: NewStateResponse) => {
    get().states.set(resp.addr, resp.state);
  }
});

export interface DeviceStateSlice {
  states: Map<BluetoothAdrr, SoundcoreDeviceState>;
  setStateFromBridgeResponse: (resp: NewStateResponse) => void;
}
