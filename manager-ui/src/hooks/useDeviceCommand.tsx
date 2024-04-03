import { BluetoothAdrr, SoundMode } from '@generated-types/soundcore-lib';
import { useAsyncBridgeRequest } from './useAsyncBridge';

// TODO: Implement using promise, waiting for the response from the async bridge
// We can generate a UUID which comes back to track the flow
export const useUpdateDeviceSoundMode = (addr: BluetoothAdrr, mode: SoundMode) => {
  useAsyncBridgeRequest({
    command: 'setSoundMode',
    payload: {
      addr: addr,
      soundMode: mode
    }
  });
};
