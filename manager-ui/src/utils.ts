import { BluetoothAdrr } from '@generated-types/soundcore-lib';

export const compareAddresses = (a: BluetoothAdrr, b: BluetoothAdrr): boolean => {
  return a.address.length === b.address.length && a.address.every((v, i) => v === b.address[i]);
};

export const bluetoothAddrToString = (addr: BluetoothAdrr): string => {
  return addr.address
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join(':')
    .toUpperCase();
};
