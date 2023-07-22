import { Observable } from "rxjs";
import {
  getSoundcoreMacPrefixes,
  matchNameToModelID,
  getUUIDSet,
  BLEConnectionUuidSet,
  getAllUUIDSets,
} from "@soundcore-lib";

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
  forcedModelId: string | undefined = undefined
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
  const modelIdFromServices =
    await searchForModelIdBasedOnServicesAndCharacteristics(gatt);

  if (
    modelIdFromServices !== undefined &&
    forcedModelId !== undefined &&
    modelIdFromServices !== forcedModelId
  ) {
    console.warn(
      `Model ID from services and characteristics (${modelIdFromServices}) does not match the supplied model ID (${forcedModelId})`
    );
  }

  if (
    (forcedModelId === undefined && gatt.device.name === null) ||
    gatt.device.name === undefined
  ) {
    console.warn(
      `Device name is null or undefined and a model ID has not been specified, cannot match model ID using name. Trying to match using services and characteristics.`
    );
  }

  const nameMatchedModelId =
    gatt.device.name !== undefined && gatt.device.name !== null
      ? // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        matchNameToModelID(gatt.device.name!)
      : undefined;

  if (
    nameMatchedModelId === undefined &&
    modelIdFromServices === undefined &&
    nameMatchedModelId !== forcedModelId
  ) {
    console.warn(
      `Name-matched model ID (${nameMatchedModelId}) does not match the Gatt matched model ID (${forcedModelId})`
    );
  }

  // Logic breakdown: If forcedModelId is undefined, use the model ID from name if it exists, otherwise use the model ID from services and characteristics if it exists.
  // If forcedModelId is defined, use it.
  const finalModelId =
    forcedModelId !== undefined
      ? forcedModelId
      : nameMatchedModelId !== undefined
      ? nameMatchedModelId
      : modelIdFromServices !== undefined
      ? modelIdFromServices
      : undefined;

  if (finalModelId === undefined) {
    gatt.disconnect();
    throw new Error(
      "Could not determine a model ID for the device, using name or services and characteristics and none was supplied"
    );
  }
  console.log(`Using model ID ${finalModelId}...`);

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

// Experimental at best :)
const searchForModelIdBasedOnServicesAndCharacteristics = async (
  gatt: BluetoothRemoteGATTServer
): Promise<string | undefined> => {
  const allServices = await gatt.getPrimaryServices();
  const allCharacteristic = await Promise.all(
    allServices.map((svc) => svc.getCharacteristics())
  );

  const deviceServiceUUIDs = allServices.map((svc) => svc.uuid);
  const deviceCharacteristicUUIDs = allCharacteristic
    .flat()
    .map((char) => char.uuid);

  const uuidMap = getAllUUIDSets() as Map<string, BLEConnectionUuidSet>;

  for (const [modelId, uuidSet] of uuidMap) {
    console.log(`Checking services and characteristics of model ${modelId}`);
    const servicesToCheck = [uuidSet.service_uuid];
    const characteristicsToCheck = [uuidSet.read_uuid, uuidSet.write_uuid];
    if (
      servicesToCheck.every((uuid) => deviceServiceUUIDs.includes(uuid)) &&
      characteristicsToCheck.every((uuid) =>
        deviceCharacteristicUUIDs.includes(uuid)
      )
    ) {
      console.log(
        `Found model ${modelId} using GATT services and characteristics`
      );
      return modelId.slice(); // Copy the string
    }
  }
  return undefined;
};
