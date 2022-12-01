import { invoke } from "@tauri-apps/api/tauri";
import { TrayDeviceStatus } from "../bindings/TrayDeviceStatus";
import { BatteryStatus } from "../bindings/BatteryStatus";
import { ANCModes } from "../bindings/ANCModes";
import { DeviceBatteryCharging, DeviceBatteryLevel, DeviceConnectionState } from "./useDeviceStore";


export interface ITrayStatus {
    deviceConnectionState: DeviceConnectionState,
    batteryLevel: DeviceBatteryLevel,
    batteryCharging: DeviceBatteryCharging,
    anc_mode: ANCModes | null,
} 

export function updateTrayStatus(data: ITrayStatus) {
    let { deviceConnectionState, batteryLevel, batteryCharging, anc_mode } = data;
    let left_batt: BatteryStatus = {
        is_charging: batteryCharging.left,
        battery_level: batteryLevel.left
    }
    let right_batt: BatteryStatus = {
        is_charging: batteryCharging.right,
        battery_level: batteryLevel.right
    }
    let status: TrayDeviceStatus = {
        is_connected: deviceConnectionState === DeviceConnectionState.CONNECTED,
        left_status: left_batt,
        right_status: right_batt,
        anc_mode: anc_mode != null ? anc_mode : "NormalMode",
    }
    invoke("set_tray_device_status", { status });
}

export function setTrayMenu(connection_state: DeviceConnectionState) {
    invoke("set_tray_menu", { isConnected: DeviceConnectionState.CONNECTED === connection_state });
}