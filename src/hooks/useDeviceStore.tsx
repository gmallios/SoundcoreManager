import { invoke } from '@tauri-apps/api/tauri';
import create from 'zustand'
import { DeviceSelection } from '../bindings/DeviceSelection';

interface DeviceStoreState {
  deviceConnectionState: DeviceConnectionState
  tryInitialize: (selectedDevice: DeviceSelection) => void,
  connectUUID: (macAddr: String, uuid: String) => void,
  getBatteryLevel: () => void,
  getBatteryCharging: () => void,

  batteryLevel: DeviceBatteryLevel,
  batteryCharging: DeviceBatteryCharging
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

export interface DeviceBatteryCharging {
  left: boolean,
  right: boolean
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
  getBatteryCharging: () => {
    invoke("get_battery_charging").then((msg: any) => {
      set((state) => ({ ...state, batteryCharging: msg }));
    }).catch((err) => {
      console.log(err);
    });
  },

  batteryLevel: { left: 0, right: 0 },
  batteryCharging: { left: false, right: false }
}))

export default useDeviceStore;