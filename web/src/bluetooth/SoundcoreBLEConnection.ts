import { Observable } from "rxjs";
import {
  getSoundcoreMacPrefixes,
  matchNameToModelID,
  getUUIDSet,
  BLEConnectionUuidSet,
  getAllUUIDSets,
} from "../../wasm/pkg/soundcore_lib_wasm";

export class SoundcoreBLEConnection {
  public readonly incomingPacketQueue: Observable<Uint8Array>;
  private gattServer: BluetoothRemoteGATTServer;
  private readCharacteristic: BluetoothRemoteGATTCharacteristic;
  private writeCharacteristic: BluetoothRemoteGATTCharacteristic;
  private _modelId: string;

  constructor(
    gatt: BluetoothRemoteGATTServer,
    readCharacteristic: BluetoothRemoteGATTCharacteristic,
    writeCharacteristic: BluetoothRemoteGATTCharacteristic,
    modelId: string
  ) {
    this.gattServer = gatt;
    this.readCharacteristic = readCharacteristic;
    this.writeCharacteristic = writeCharacteristic;
    this._modelId = modelId;

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

  public get name(): string {
    return this.gattServer.device.name || "";
  }

  public get modelId(): string {
    return this._modelId;
  }
}

export const connectToSoundcoreDevice = async (
  modelid: string | undefined = undefined
): Promise<SoundcoreBLEConnection> => {
  const uuidSetsForScan = [];
  getAllUUIDSets().forEach((set: BLEConnectionUuidSet) => {
    uuidSetsForScan.push(set.service_uuid);
  });
  uuidSetsForScan.push(
    ...[
      "00001800-0000-1000-8000-00805f9b34fb",
      "00001801-0000-1000-8000-00805f9b34fb",
      "011af5da-0000-1000-8000-00805f9b34fb",
      "66666666-6666-6666-6666-666666666666",
      "86868686-8686-8686-8686-868686868686",
    ]
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
    optionalServices: uuidSetsForScan,
  });

  if (device.gatt === undefined) {
    throw new Error("Device does not support GATT");
  }

  const gatt = await device.gatt.connect();

  if (
    (modelid === undefined && gatt.device.name === null) ||
    gatt.device.name === undefined
  ) {
    device.gatt?.disconnect();
    // TODO: Determine model from other means such as MAC or service UUID
    throw new Error(
      "Device name is null and model is unspecified, can't determine the model"
    );
  }

  const finalModelId = modelid ? modelid : matchNameToModelID(gatt.device.name);

  const uuidSet: BLEConnectionUuidSet = getUUIDSet(finalModelId);
  const bleService = await gatt.getPrimaryService(uuidSet.service_uuid);
  const [readCharacteristic, writeCharacteristic] = await Promise.all([
    bleService.getCharacteristic(uuidSet.read_uuid),
    bleService.getCharacteristic(uuidSet.write_uuid),
  ]);

  await readCharacteristic.startNotifications();

  return new SoundcoreBLEConnection(
    gatt,
    readCharacteristic,
    writeCharacteristic,
    finalModelId
  );
};
