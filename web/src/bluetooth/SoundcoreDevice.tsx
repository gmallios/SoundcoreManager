import { BehaviorSubject, Subscription, first, firstValueFrom, interval, map, takeUntil } from "rxjs";
import { SoundcoreBLEConnection, connectToSoundcoreDevice } from "@bluetooth/SoundcoreBLEConnection";
import { EQValues, SoundMode, SoundcoreDeviceState, get_state_update_packet, handle_byte_response } from "@soundcore-lib";


export interface ISoundcoreDevice {
    disconnect(): void;
    name: string;
    state: Omit<BehaviorSubject<SoundcoreDeviceState>, "next" | "error" | "complete">;
    modelId: string;
    requestNewState(): void;
    set_sound_mode(mode: SoundMode): void;
    set_eq(eq: EQValues): void;
}

export class SoundcoreDevice implements ISoundcoreDevice {
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

export class SoundcoreDeviceMock implements ISoundcoreDevice {
    private readonly _state: BehaviorSubject<SoundcoreDeviceState>;
    private readonly _modelId: string;

    public constructor() {
        this._modelId = "MockedModelId";
        this._state = new BehaviorSubject<SoundcoreDeviceState>(new SoundcoreDeviceState());
    }

    public disconnect(): void {
        // Do nothing
    }

    public get name(): string {
        return "Soundcore Mocked Device";
    }

    public get state(): Omit<BehaviorSubject<SoundcoreDeviceState>, "next" | "error" | "complete"> {
        return this._state;
    }

    public get modelId(): string {
        return this._modelId;
    }

    public requestNewState(): void {
        const newState = this._state.value;
        newState.battery_level.left = Math.random() * 5;
        newState.battery_level.right = Math.random() * 5;
        newState.sound_mode = Math.random() > 0.5 ? new SoundMode("NoiseCancelling(Outdoor)") : new SoundMode("NormalMode");
        this._state.next(newState);
    }

    public set_sound_mode(mode: SoundMode): void {
        const newState = this._state.value;
        newState.sound_mode = mode;
        this._state.next(newState);
    }

    public set_eq(eq: EQValues): void {
        const newState = this._state.value;
        newState.eq = eq;
        this._state.next(newState);
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
            const newState = handle_byte_response(packet, connection.modelId, emptyState);
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