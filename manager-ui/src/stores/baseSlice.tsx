import { Event } from '@tauri-apps/api/event';
import { StateCreator, StoreApi } from 'zustand';
import { SoundcoreStoreSlices } from './useSoundcoreStore';
import { BridgeResponse } from '../types/tauri-backend';

export interface BaseSlice {
  handleAsyncBridgeEvent: (e: Event<BridgeResponse>) => void;
}

export const createBaseSlice: StateCreator<SoundcoreStoreSlices, [], [], BaseSlice> = (
  set,
  get
) => ({
  handleAsyncBridgeEvent: (e: Event<BridgeResponse>) => {
    const payload = e.payload as BridgeResponse;
    const kind = e.payload.kind as BridgeResponse['kind'];
    const handler = bridgeResponseHandlers[kind];
    if (handler) {
      handler(payload.payload, set, get);
    }
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
  scanResult: function (payload, _set, get): void {
    get().setLatestScan(payload);
  },
  connectionEstablished: (_e, _set, _get): void => {
    throw new Error('Function not implemented.');
  },
  disconnected: (_e, _set, _get): void => {
    throw new Error('Function not implemented.');
  },
  error: (_e, _set, _get): void => {
    throw new Error('Function not implemented.');
  },
  adapterEvent: (_payload, _set, _get) => {
    throw new Error('Function not implemented.');
  }
};
