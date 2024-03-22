import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { BluetoothAddrSet } from './addrSet';

describe('BluetoothAddrSet', () => {
  it('should add', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    expect(addrSet.size).toBe(1);
    expect(addrSet.values).toEqual(['01:02:03:04:05:06']);
  });

  it('should not add duplicate', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    addrSet.add(addr);
    expect(addrSet.size).toBe(1);
  });

  it('should remove', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    addrSet.remove(addr);
    expect(addrSet.size).toBe(0);
  });

  it('should have', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    expect(addrSet.has(addr)).toBe(true);
  });

  it('should get values', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    expect(addrSet.values).toEqual(['01:02:03:04:05:06']);
  });

  it('should clear', () => {
    const addrSet = new BluetoothAddrSet();
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    addrSet.clear();
    expect(addrSet.size).toBe(0);
  });

  it('should have size', () => {
    const addrSet = new BluetoothAddrSet();
    expect(addrSet.size).toBe(0);
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    addrSet.add(addr);
    expect(addrSet.size).toBe(1);
  });
});
