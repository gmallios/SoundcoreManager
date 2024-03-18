import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { bluetoothAddrToString, compareAddresses } from './utils';

describe('BluetoothAddr tests', () => {
  it('should compare addresses', () => {
    const addrA: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    const addrB: BluetoothAdrr = { address: [82, 62, 138, 87, 42, 54] };
    const addrC: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    expect(compareAddresses(addrA, addrB)).toBe(false);
    expect(compareAddresses(addrA, addrC)).toBe(true);
  });

  it('should convert to string', () => {
    const addr: BluetoothAdrr = { address: [83, 64, 158, 87, 12, 84] };
    expect(bluetoothAddrToString(addr)).toBe('53:40:9E:57:0C:54');
  });
});
