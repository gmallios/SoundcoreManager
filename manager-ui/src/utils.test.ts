import { BluetoothAdrr } from '@generated-types/soundcore-lib';
import { bluetoothAddrToString, compareAddresses } from './utils';

describe('BluetoothAddr tests', () => {
  it('should compare addresses', () => {
    const addrA: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    const addrB: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    expect(compareAddresses(addrA, addrB)).toBe(true);
  });

  it('should convert to string', () => {
    const addr: BluetoothAdrr = { address: [1, 2, 3, 4, 5, 6] };
    expect(bluetoothAddrToString(addr)).toBe('01:02:03:04:05:06');
  });
});
