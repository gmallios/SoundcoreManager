import { invoke } from '@tauri-apps/api/tauri';
import create from 'zustand'
import { DeviceSelection } from '../bindings/DeviceSelection';

interface DeviceStoreState {
  deviceConnectionState: DeviceConnectionState
  tryInitialize: (selectedDevice: DeviceSelection) => void,
  connectUUID: (macAddr: String, uuid: String) => void,
  getBatteryLevel: () => void,

  batteryLevel: DeviceBatteryLevel
}

export enum DeviceConnectionState {
  DISCONNECTED,
  CONNECTED,
  INITIALIZED,
  UNINITIALIZED
}

export interface DeviceBatteryLevel {
  left: number,
  right: number
}

const useDeviceStore = create<DeviceStoreState>((set) => ({
  deviceConnectionState: DeviceConnectionState.UNINITIALIZED,

  tryInitialize: (selectedDevice: DeviceSelection) => {
    invoke("init_device", { device: selectedDevice }).then((_msg) => {
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.INITIALIZED }));
    }).catch((err) => {
      console.log(err);
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.UNINITIALIZED }));
    });
  },
  connectUUID: (macAddr: String, uuid: String) => {
    invoke("connect_uuid", { macAddr: macAddr, uuid: uuid }).then((_msg) => {
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.CONNECTED }));
    }).catch((err) => {
      console.log(err);
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.UNINITIALIZED }));
    });
  },
  getBatteryLevel: () => {
    invoke("get_battery_level").then((msg: any) => {
      set((state) => ({ ...state, batteryLevel: msg }));
    }).catch((err) => {
      console.log(err);
    });
  },

  batteryLevel: { left: 0, right: 0 }
}))

export default useDeviceStore;