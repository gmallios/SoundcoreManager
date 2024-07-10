import { DiscoveredDevice, BluetoothAdrr } from '@generated-types/soundcore-lib';
import { useAsyncBridgeRequest } from '@hooks/useAsyncBridge';
import { BluetoothAddrKeyedMap } from '@utils/addrMap';
import { BluetoothAddrSet } from '@utils/addrSet';
import { StateCreator } from 'zustand';

export const createBluetoothSlice: StateCreator<BluetoothSlice> = (set, _get) => ({
  latestScan: null,
  isScanLoading: false,
  hasScanFailed: false,
  connectedAddresses: new BluetoothAddrSet(),
  failedConnectionMap: new BluetoothAddrKeyedMap<string>(),
  setLatestScan: (scanRes: DiscoveredDevice[]) =>
    set({ latestScan: scanRes, isScanLoading: false }),
  startScan: () => {
    useAsyncBridgeRequest({ command: 'scan' })
      .then(() => {
        set({ isScanLoading: true });
      })
      .catch((error) => {
        console.error(`Could not start scan. ${error}`);
        set({ hasScanFailed: true });
      });
  },
  connectDevice: (device: DiscoveredDevice) => {
    useAsyncBridgeRequest({ command: 'connect', payload: device }).catch((error) => {
      console.error(`Could not connect to device. ${error}`);
    });
  },
  addConnectedDevice: (addr: BluetoothAdrr) => {
    set((state) => {
      const set = new BluetoothAddrSet(state.connectedAddresses.values);
      set.add(addr);
      return {
        connectedAddresses: set
      };
    });
  },
  removeConnectedDevice: (addr: BluetoothAdrr) => {
    set((state) => {
      const set = new BluetoothAddrSet(state.connectedAddresses.values);
      set.remove(addr);
      return {
        connectedAddresses: set
      };
    });
  },
  addFailedConnection: (addr: BluetoothAdrr, reason: string) => {
    set((state) => {
      const map = new BluetoothAddrKeyedMap<string>(state.failedConnectionMap.entries());
      map.set(addr, reason);
      return {
        failedConnectionMap: map
      };
    });
  },
  removeFailedConnection: (addr: BluetoothAdrr) => {
    set((state) => {
      const map = new BluetoothAddrKeyedMap<string>(state.failedConnectionMap.entries());
      map.delete(addr);
      return {
        failedConnectionMap: map
      };
    });
  }
});

export interface BluetoothSlice {
  latestScan: DiscoveredDevice[] | null;
  isScanLoading: boolean;
  hasScanFailed: boolean;
  connectedAddresses: BluetoothAddrSet;
  failedConnectionMap: BluetoothAddrKeyedMap<string>;
  setLatestScan: (scanRes: DiscoveredDevice[]) => void;
  startScan: () => void;
  connectDevice: (device: DiscoveredDevice) => void;
  addConnectedDevice: (addr: BluetoothAdrr) => void;
  removeConnectedDevice: (addr: BluetoothAdrr) => void;
  addFailedConnection: (addr: BluetoothAdrr, reason: string) => void;
  removeFailedConnection: (addr: BluetoothAdrr) => void;
}
