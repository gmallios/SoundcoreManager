import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { bluetoothAddrToString } from './utils';

export class BluetoothAddrSet {
  private addrs: Set<string> = new Set();

  constructor(addrs?: string[]) {
    if (addrs) {
      addrs.forEach((addr) => {
        this.addrs.add(addr);
      });
    }
  }

  add(addr: BluetoothAdrr): void {
    this.addrs.add(bluetoothAddrToString(addr));
  }

  remove(addr: BluetoothAdrr): void {
    this.addrs.delete(bluetoothAddrToString(addr));
  }

  has(addr: BluetoothAdrr): boolean {
    return this.addrs.has(bluetoothAddrToString(addr));
  }

  get size(): number {
    return this.addrs.size;
  }

  get values(): string[] {
    return Array.from(this.addrs);
  }

  clear(): void {
    this.addrs.clear();
  }
}
