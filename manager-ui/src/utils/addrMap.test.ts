import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { BluetoothAddrKeyedMap } from './addrMap';

describe('BluetoothAddrKeyedMap', () => {
  it('should set and get values', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrB, 2);
    expect(map.get(addrA)).toBe(1);
    expect(map.get(addrB)).toBe(2);
  });

  it('should update values with same address', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrA, 2);
    expect(map.get(addrA)).toBe(2);
  });

  it('should delete values', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.delete(addrA);
    expect(map.get(addrA)).toBe(undefined);
  });

  it('should clear all values', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrB, 2);
    map.clear();
    expect(map.size).toBe(0);
  });

  it('should get size', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrB, 2);
    expect(map.size).toBe(2);
  });

  it('should get keys', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrB, 2);
    expect(map.keys()).toEqual([addrA, addrB]);
  });

  it('should get values', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const map = new BluetoothAddrKeyedMap<number>();
    map.set(addrA, 1);
    map.set(addrB, 2);
    expect(map.values()).toEqual([1, 2]);
  });
});
