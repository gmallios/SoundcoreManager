import { WebBLEDevice } from '@wasm/manager_wasm';
import { EQProfile, SoundcoreDeviceState, SoundMode } from '@generated-types/soundcore-lib';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';

export class BLEDevice {
  private readonly webBLEDevice: WebBLEDevice;
  private readonly device: BluetoothDevice;

  constructor(webBLEDevice: WebBLEDevice, device: BluetoothDevice) {
    this.webBLEDevice = webBLEDevice;
    this.device = device;

    void webBLEDevice.setOnStateChange((state: SoundcoreDeviceState) => {
      useWebManagerStore.getState().setCurrentState(state);
    });

    webBLEDevice.latestState().then((state: SoundcoreDeviceState) => {
      useWebManagerStore.getState().setCurrentState(state);
    });
  }

  public async setSoundMode(soundMode: SoundMode): Promise<void> {
    return this.webBLEDevice.setSoundMode(JSON.stringify(soundMode));
  }

  public async setEqualizerCustom(values: number[]): Promise<void> {
    return this.webBLEDevice.setEqualizerCustom(new Int8Array(values));
  }

  public async setEqualizerPreset(profile: EQProfile): Promise<void> {
    return this.webBLEDevice.setEqualizerPreset(JSON.stringify(profile));
  }
}

export const BLEDeviceFactory = async (device: BluetoothDevice): Promise<BLEDevice> => {
  const webBleDevice = await WebBLEDevice.new(device);
  return new BLEDevice(webBleDevice, device);
};