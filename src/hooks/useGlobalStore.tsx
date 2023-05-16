import { invoke } from '@tauri-apps/api/tauri';
import { create } from 'zustand';
import { SupportedModels } from '../types/soundcore-lib';

interface DeviceStoreState {
  deviceConnectionState: DeviceConnectionState
  deviceModel: SupportedModels | undefined,
  updateDeviceModel: (model: SupportedModels) => void,
  setDeviceConnectionState: (state: DeviceConnectionState) => void,
}

export enum DeviceConnectionState {
  DISCONNECTED,
  CONNECTING,
  CONNECTED,
}

//TODO: Export this from rust 
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

const useGlobalStore = create<DeviceStoreState>((set) => ({
  deviceConnectionState: DeviceConnectionState.DISCONNECTED,
  deviceModel: undefined,
  updateDeviceModel: (model: SupportedModels) => {
    set((state) => ({ ...state, deviceModel: model }));
  },
  setDeviceConnectionState: (new_state: DeviceConnectionState) => {
    set((state) => ({ ...state, deviceConnectionState: new_state }));
  },
}))

export default useGlobalStore;