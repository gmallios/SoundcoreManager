
interface ISoundcoreDevice {
    get state(); // TODO: Add type
    get name(): string | undefined;
    get address(): string | undefined;
    get modelid(): string | undefined;
    get soundMode(); // TODO: Add type
    get eq(); // TODO: Add type
    get batteryLevel(); // TODO: Add type
    get chargingStatus(); // TODO: Add type
    disconnect(): void; 
    sendState(new): Promise<void>; // TODO: Add type
}

export class SoundcoreDevice implements ISoundcoreDevice {
    private readonly connection; 
}