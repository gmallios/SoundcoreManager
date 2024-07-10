import { create } from 'zustand';
import { createDeviceStateSlice, DeviceStateSlice } from './deviceStateSlice';
import { BaseSlice, createBaseSlice } from './baseSlice';
import { BluetoothSlice, createBluetoothSlice } from './bluetoothSlice';

export type TauriManagerStoreSlices = BaseSlice & DeviceStateSlice & BluetoothSlice;

export const useTauriManagerStore = create<TauriManagerStoreSlices>()((...a) => ({
  ...createDeviceStateSlice(...a),
  ...createBaseSlice(...a),
  ...createBluetoothSlice(...a)
}));
