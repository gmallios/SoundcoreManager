import { invoke } from '@tauri-apps/api/tauri';
import create from 'zustand'
import { ANCModes } from '../bindings/ANCModes';
import { DeviceSelection } from '../bindings/DeviceSelection';

interface DeviceStoreState {
  deviceConnectionState: DeviceConnectionState
  tryInitialize: (selectedDevice: DeviceSelection) => void,
  connectUUID: (macAddr: String, uuid: String) => void,
  // Get functions
  getBatteryLevel: () => void,
  getBatteryCharging: () => void,
  getANCMode: () => void,
  // Set functions
  sendANCMode: (mode: ANCModes) => void,
  // Earbud state
  currentANCMode: ANCModes | null,
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

  getANCMode: () => {
    invoke("get_anc_mode").then((msg: any) => {
      let mode = msg as ANCModes;
      set((state) => ({ ...state, currentANCMode: mode }));
    }).catch((err) => {
      console.log(err);
    });
  },

  sendANCMode: (mode: ANCModes) => {
    invoke("set_anc_mode", { mode: mode }).then((_msg) => {
      set((state) => ({ ...state, currentANCMode: mode }));
    }).catch((err) => {
      console.log(err);
    });
  },

  currentANCMode: null,
  batteryLevel: { left: 0, right: 0 },
  batteryCharging: { left: false, right: false }
}))

export default useDeviceStore;