import { WebBLEDevice } from '@wasm/manager_wasm';
import { SoundcoreDeviceState } from '@generated-types/soundcore-lib';
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
}