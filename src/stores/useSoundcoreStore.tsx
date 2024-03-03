import { create } from 'zustand';
import { DeviceStateSlice, createDeviceStateSlice } from './deviceStateSlice';
import { createBaseSlice, BaseSlice } from './baseSlice';
import { BluetoothSlice, createBluetoothSlice } from './bluetoothSlice';

export type SoundcoreStoreSlices = BaseSlice & DeviceStateSlice & BluetoothSlice;

export const useSoundcoreStore = create<SoundcoreStoreSlices>()((...a) => ({
  ...createDeviceStateSlice(...a),
  ...createBaseSlice(...a),
  ...createBluetoothSlice(...a)
}));
