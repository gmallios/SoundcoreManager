import { WebBLEDevice } from '@wasm/manager_wasm';
import { EQProfile, SoundcoreDeviceState, SoundMode } from '@generated-types/soundcore-lib';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';

export class BLEDevice {
  private readonly webBLEDevice: WebBLEDevice;

  constructor(webBLEDevice: WebBLEDevice) {
    this.webBLEDevice = webBLEDevice;

    void webBLEDevice.setOnStateChange((state: SoundcoreDeviceState) => {
      console.log('onStateChange');
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