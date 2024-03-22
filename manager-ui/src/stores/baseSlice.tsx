import { Event } from '@tauri-apps/api/event';
import { StateCreator, StoreApi } from 'zustand';
import { SoundcoreStoreSlices } from './useSoundcoreStore';
import { BridgeResponse } from '../types/tauri-backend';
import { BluetoothAdrr, SoundcoreDeviceState } from '@generated-types/soundcore-lib';

export interface BaseSlice {
  currentViewedDevice: BluetoothAdrr | null;
  setCurrentViewedDevice: (addr: BluetoothAdrr) => void;
  handleAsyncBridgeEvent: (e: Event<BridgeResponse>) => void;
  currentViewedDeviceState: () => SoundcoreDeviceState | null;
}

export const createBaseSlice: StateCreator<SoundcoreStoreSlices, [], [], BaseSlice> = (
  set,
  get
) => ({
  currentViewedDevice: null,
  setCurrentViewedDevice: (addr) => {
    set({ currentViewedDevice: addr });
  },
  handleAsyncBridgeEvent: (e: Event<BridgeResponse>) => {
    console.log('Handling event', e);
    const payload = e.payload as BridgeResponse;
    const kind = e.payload.kind as BridgeResponse['kind'];
    const handler = bridgeResponseHandlers[kind];
    if (handler) {
      handler(payload.payload, set, get);
    } else {
      console.warn(`No handler for event kind: ${kind} with payload: ${payload}`);
    }
  },
  currentViewedDeviceState: () => {
    const addr = get().currentViewedDevice;
    if (!addr) {
      return null;
    }
    return get().states.get(addr) || null;
  }
});

type BridgeResponseHandlers = {
  [K in BridgeResponse['kind']]: (
    e: Extract<BridgeResponse, { kind: K }>['payload'],
    set: StoreApi<SoundcoreStoreSlices>['setState'],
    get: StoreApi<SoundcoreStoreSlices>['getState']
  ) => void;
};

const bridgeResponseHandlers: BridgeResponseHandlers = {
  newState: (payload, _set, get) => {
    get().setStateFromBridgeResponse(payload);
  },
  scanResult: (payload, _set, get): void => {
    get().setLatestScan(payload);
  },
  connectionEstablished: (e, _set, get): void => {
    get().addConnectedDevice(e.addr);
    get().setStateFromBridgeResponse(e);

    if (get().currentViewedDevice === null) {
      get().setCurrentViewedDevice(e.addr);
    }
  },
  connectionFailed: (e, _set, get): void => {
    get().addFailedConnection(e.addr, e.reason);
  },
  disconnected: (_e, _set, _get): void => {
    throw new Error('Function not implemented.');
  },
  genericError: (_e, _set, _get): void => {
    throw new Error('Function not implemented.');
  },
  adapterEvent: (_payload, _set, _get) => {
    throw new Error('Function not implemented.');
  },
  disconnectedAll: (_payload, _set, _get) => {
    // TODO: Wait for this beforing "starting" the app? No-op for now.
  }
};
