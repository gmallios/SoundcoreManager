import { invoke } from '@tauri-apps/api/tauri';
import create from 'zustand'
import { ANCModes } from '../bindings/ANCModes';
import { DeviceSelection } from '../bindings/DeviceSelection';

interface DeviceStoreState {
  deviceConnectionState: DeviceConnectionState
  setDeviceConnectionState: (state: DeviceConnectionState) => void,
  connectUUID: (selection: DeviceSelection, addr: String) => void,
  // Get functions
  getBatteryLevel: () => void,
  getBatteryCharging: () => void,
  getANCMode: () => void,
  getDeviceStatus: () => void,
  // Set functions
  sendANCMode: (mode: ANCModes) => void,
  sendEQ: (eq: EQWave) => void,
  // Earbud state
  currentANCMode: ANCModes | null,
  batteryLevel: DeviceBatteryLevel,
  batteryCharging: DeviceBatteryCharging
  deviceStatus: DeviceStatus | null
}

export enum DeviceConnectionState {
  DISCONNECTED,
  CONNECTING,
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

export interface DeviceInfo {
  left_fw: string,
  right_fw: string,
  SN: string,
}

export interface DeviceStatus {
  host_device: number,
  tws_status: boolean,
  battery_level: DeviceBatteryLevel,
  battery_charging: DeviceBatteryCharging,
  anc_mode: ANCValues,
  side_tone_enabled: boolean,
  wear_detection_enabled: boolean,
  touch_tone_enabled: boolean,
  left_eq: EQWave,
  right_eq: EQWave,
  hearid_enabled: boolean,
  left_hearid: EQWave,
  right_hearid: EQWave,
  left_hearid_customdata: EQWave,
  right_hearid_customdata: EQWave,
}

export interface ANCValues {
  option: number,
  anc_option: number,
  transparency_option: number,
  anc_custom: number,
}

export interface EQWave {
  pos0: number,
  pos1: number,
  pos2: number,
  pos3: number,
  pos4: number,
  pos5: number,
  pos6: number,
  pos7: number,
  pos8: number,
  pos9: number,
}
/* Move to React Query? */
const useDeviceStore = create<DeviceStoreState>((set) => ({
  deviceConnectionState: DeviceConnectionState.UNINITIALIZED,
  
  setDeviceConnectionState: (new_state: DeviceConnectionState) => {
    set((state) => ({ ...state, deviceConnectionState: new_state }));
  },
  connectUUID: (selection: DeviceSelection, addr: String) => {
    set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.CONNECTING }));
    invoke("connect", { selection: selection, addr: addr }).then((_msg) => {
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.CONNECTED }));
    }).catch((err) => {
      console.log(err);
      set((state) => ({ ...state, deviceConnectionState: DeviceConnectionState.DISCONNECTED }));
    });
  },
  getDeviceStatus: () => {
    invoke("get_status").then((msg) => {
      set((state) => ({ ...state, deviceStatus: msg as DeviceStatus }));
    }).catch((err) => {
      console.log(err);
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
    invoke("get_anc").then((msg: any) => {
      let mode = msg as ANCModes;
      set((state) => ({ ...state, currentANCMode: mode }));
    }).catch((err) => {
      console.log(err);
    });
  },

  sendANCMode: (mode: ANCModes) => {
    invoke("set_anc", { mode: mode }).then((_msg) => {
      set((state) => ({ ...state, currentANCMode: mode }));
    }).catch((err) => {
      console.log(err);
    });
  },
  sendEQ: (eq: EQWave) => {
    invoke("set_eq", { eq }).then((_msg) => {
      set((state) => {
        let newState = { ...state };
        if (newState.deviceStatus != null) {
          newState.deviceStatus.left_eq = eq;
          newState.deviceStatus.right_eq = eq;
        }
        return newState;
      })
    }).catch((err) => {
      console.log(err);
    });
  },


  currentANCMode: null,
  batteryLevel: { left: 0, right: 0 },
  batteryCharging: { left: false, right: false },
  deviceStatus: null
}))

export default useDeviceStore;