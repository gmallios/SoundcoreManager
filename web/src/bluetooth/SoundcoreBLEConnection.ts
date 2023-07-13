import { Observable } from "rxjs";
import {
  getSoundcoreMacPrefixes,
  matchNameToModelID,
  getUUIDSet,
  BLEConnectionUuidSet,
  getAllUUIDSets,
} from "../../wasm/pkg/soundcore_lib_wasm";

export class SoundcoreBLEConnection {
  private readonly incomingPacketQueue: Observable<Uint8Array>;
  private gattServer: BluetoothRemoteGATTServer;
  private readCharacteristic: BluetoothRemoteGATTCharacteristic;
  private writeCharacteristic: BluetoothRemoteGATTCharacteristic;

  constructor(
    gatt: BluetoothRemoteGATTServer,
    readCharacteristic: BluetoothRemoteGATTCharacteristic,
    writeCharacteristic: BluetoothRemoteGATTCharacteristic
  ) {
    this.gattServer = gatt;
    this.readCharacteristic = readCharacteristic;
    this.writeCharacteristic = writeCharacteristic;

    this.incomingPacketQueue = new Observable((subscriber) => {
      // Handler for incoming packets
      const handler = () => {
        if (this.readCharacteristic.value) {
          subscriber.next(new Uint8Array(this.readCharacteristic.value.buffer));
        } else {
          console.error("Read characteristic value is null");
        }
      };
      // Add handler to read characteristic
      this.readCharacteristic.addEventListener(
        "characteristicvaluechanged",
        handler
      );
      // Remove on cleanup
      return () => {
        readCharacteristic.removeEventListener(
          "characteristicvaluechanged",
          handler
        );
      };
    });
  }

  public get connected(): boolean {
    return this.gattServer.connected;
  }

  public disconnect() {
    this.readCharacteristic.removeEventListener(
      "characteristicvaluechanged",
      null
    );
    this.gattServer.disconnect();
  }

  public async write(value: BufferSource, withoutResponse = false) {
    if (withoutResponse) {
      await this.writeCharacteristic.writeValueWithoutResponse(value);
      return;
    }
    await this.writeCharacteristic.writeValue(value);
  }
}

export const connectToSoundcoreDevice =
  async (): Promise<SoundcoreBLEConnection> => {
    const allSupportedServices = getAllUUIDSets().map(
      (uuidSet: BLEConnectionUuidSet) => uuidSet.service_uuid
    );
    const macAddrPrefixes = getSoundcoreMacPrefixes();
    const device = await navigator.bluetooth.requestDevice({
      filters: macAddrPrefixes.map((prefix) => ({
        manufacturerData: [
          {
            companyIdentifier: (prefix[1] << 8) | prefix[0],
            dataPrefix: Uint8Array.of(prefix[2]),
          },
        ],
      })),
      optionalServices: allSupportedServices,
    });

    if (device.gatt === undefined) {
      throw new Error("Device does not support GATT");
    }

    const gatt = await device.gatt.connect();

    if (gatt.device.name === null || gatt.device.name === undefined) {
      // TODO: Determine model from other means such as MAC or service UUID
      throw new Error("Device name is null, can't determine the model");
    }

    const modelid = matchNameToModelID(gatt.device.name);
    const uuidSet: BLEConnectionUuidSet = getUUIDSet(modelid);
    const bleService = await gatt.getPrimaryService(uuidSet.service_uuid);
    const [readCharacteristic, writeCharacteristic] = await Promise.all([
      bleService.getCharacteristic(uuidSet.read_uuid),
      bleService.getCharacteristic(uuidSet.write_uuid),
    ]);

    await readCharacteristic.startNotifications();

    return new SoundcoreBLEConnection(
      gatt,
      readCharacteristic,
      writeCharacteristic
    );
  };
