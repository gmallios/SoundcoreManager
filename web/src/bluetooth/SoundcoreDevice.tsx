import { BehaviorSubject, Subscription, first, firstValueFrom, interval, map, takeUntil } from "rxjs";
import { SoundcoreBLEConnection, connectToSoundcoreDevice } from "./SoundcoreBLEConnection";
import { SoundcoreDeviceState, get_state_update_packet, handle_byte_response } from "../../wasm/pkg/soundcore_lib_wasm";

export class SoundcoreDevice {
    private readonly connection: SoundcoreBLEConnection;
    private readonly incomingMessageSubscription: Subscription;
    private readonly _state: BehaviorSubject<SoundcoreDeviceState>;
    private readonly _modelId: string;

    public constructor(connection: SoundcoreBLEConnection, state: SoundcoreDeviceState, modelid: string) {
        this.connection = connection;
        this._state = new BehaviorSubject<SoundcoreDeviceState>(state);
        this._modelId = modelid;
        this.incomingMessageSubscription = this.connection.incomingPacketQueue.subscribe(this.packetHandler);
    }

    public disconnect(): void {
        this.incomingMessageSubscription.unsubscribe();
        this.connection.disconnect();
    }

    public get name(): string {
        return this.connection.name;
    }

    public get state(): Omit<BehaviorSubject<SoundcoreDeviceState>, "next" | "error" | "complete"> {
        return this._state;
    }

    public get modelId(): string {
        return this._modelId;
    }

    private packetHandler: (packet: Uint8Array) => void = (packet: Uint8Array) => {
        this._state.next(handle_byte_response(packet, this.modelId, this._state.value));
    }

    public requestNewState(): void {
        const stateUpdatePacket = get_state_update_packet(this.modelId);
        this.connection.write(stateUpdatePacket);
    }

}

export async function selectDevice(modelid: string | undefined = undefined): Promise<SoundcoreDevice> {
    const connection = await connectToSoundcoreDevice(modelid);
    return createSoundcoreDevice(connection);
}

export const createSoundcoreDevice = async (connection: SoundcoreBLEConnection): Promise<SoundcoreDevice> => {
    const stateUpdatePacket = get_state_update_packet(connection.modelId);
    const emptyState = new SoundcoreDeviceState();

    // Ideally this filters only the state update responses
    const pipe = connection.incomingPacketQueue.pipe(
        map((packet: Uint8Array) => {
            console.log("Got packet: ", packet);
            const newState = handle_byte_response(packet, connection.modelId, emptyState);
            console.log("New state: ", newState);
            return newState;
        }),
        takeUntil(interval(5000)),
        first()
    );


    const [, initialState] = await Promise.all([
        connection.write(stateUpdatePacket),
        firstValueFrom(pipe)
    ]);

    return new SoundcoreDevice(connection, initialState, connection.modelId);
}