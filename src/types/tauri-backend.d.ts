/*
 Generated by typeshare 1.7.0
*/

export interface NewStateResponse {
	addr: BluetoothAdrr;
	state: SoundcoreDeviceState;
}

export interface BthScanResult {
	name: string;
	address: string;
	is_connected: boolean;
}

export type ANCModes = 
	| { mode: "NormalMode", value?: undefined }
	| { mode: "AncTransportMode", value?: undefined }
	| { mode: "AncOutdoorMode", value?: undefined }
	| { mode: "AncIndoorMode", value?: undefined }
	| { mode: "AncCustomValue", value: number }
	| { mode: "TransparencyFullyTransparentMode", value?: undefined }
	| { mode: "TransparencyVocalMode", value?: undefined };

export interface NewTrayDeviceStatus {
	is_connected: boolean;
	charging: BatteryCharging;
	level: BatteryLevel;
	anc_mode: ANCModes;
}

export enum SupportedANCProfiles {
	Normal = "Normal",
	AncTransportMode = "AncTransportMode",
	AncOutdoorMode = "AncOutdoorMode",
	AncIndoorMode = "AncIndoorMode",
	AncCustomValue = "AncCustomValue",
	TransparencyFullyTransparentMode = "TransparencyFullyTransparentMode",
	TransparencyVocalMode = "TransparencyVocalMode",
}

export interface DeviceFeatures {
	profiles: SupportedANCProfiles[];
}

export type BridgeCommand = 
	| { command: "scan", payload?: undefined }
	| { command: "connect", payload: DiscoveredDevice }
	| { command: "disconnect", payload: BluetoothAdrr };

export type BridgeResponse = 
	| { kind: "scanResult", payload: DiscoveredDevice[] }
	| { kind: "connectionEstablished", payload: BluetoothAdrr }
	| { kind: "newState", payload: NewStateResponse }
	| { kind: "disconnected", payload: BluetoothAdrr }
	| { kind: "error", payload: string };

