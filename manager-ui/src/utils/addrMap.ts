import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { bluetoothAddrToString } from './utils';

export class BluetoothAddrKeyedMap<V> {
  private map: Map<string, V> = new Map();

  public constructor(entries?: [BluetoothAdrr, V][]) {
    if (entries) {
      entries.forEach(([addr, value]) => {
        this.set(addr, value);
      });
    }
  }

  public set(addr: BluetoothAdrr, value: V): void {
    this.map.set(bluetoothAddrToString(addr), value);
  }

  public get(addr: BluetoothAdrr): V | undefined {
    return this.map.get(bluetoothAddrToString(addr));
  }

  public has(addr: BluetoothAdrr): boolean {
    return this.map.has(bluetoothAddrToString(addr));
  }

  public delete(addr: BluetoothAdrr): void {
    this.map.delete(bluetoothAddrToString(addr));
  }

  public clear(): void {
    this.map.clear();
  }

  public get size(): number {
    return this.map.size;
  }

  public keys(): BluetoothAdrr[] {
    return Array.from(this.map.keys()).map((key) => {
      const addr = key.split(':').map((byte) => parseInt(byte, 16));
      return { address: addr } as BluetoothAdrr;
    });
  }

  public values(): V[] {
    return Array.from(this.map.values());
  }

  public entries(): [BluetoothAdrr, V][] {
    return Array.from(this.map.entries()).map(([key, value]) => {
      const addr = key.split(':').map((byte) => parseInt(byte, 16));
      return [{ address: addr }, value] as [BluetoothAdrr, V];
    });
  }
}
