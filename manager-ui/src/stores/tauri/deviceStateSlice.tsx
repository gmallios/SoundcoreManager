import { StateCreator } from 'zustand';
import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';
import { TaggedStateResponse } from '@generated-types/tauri-backend';
import { BluetoothAddrKeyedMap } from '@utils/addrMap';

export const createDeviceStateSlice: StateCreator<DeviceStateSlice> = (set, _get) => ({
  states: new BluetoothAddrKeyedMap<SoundcoreDeviceState>(),
  setStateFromBridgeResponse: (resp: TaggedStateResponse) => {
    set((state) => {
      const newMap = new BluetoothAddrKeyedMap<SoundcoreDeviceState>(state.states.entries());
      newMap.set(resp.addr, resp.state);
      return { states: newMap };
    });
  }
});

export interface DeviceStateSlice {
  states: BluetoothAddrKeyedMap<SoundcoreDeviceState>;
  setStateFromBridgeResponse: (resp: TaggedStateResponse) => void;
}
