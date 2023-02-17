import { invoke } from "@tauri-apps/api/tauri";
import { TrayDeviceStatus } from "../bindings/TrayDeviceStatus";
import { BatteryStatus } from "../bindings/BatteryStatus";
import { ANCModes } from "../bindings/ANCModes";
import { DeviceBatteryCharging, DeviceBatteryLevel, DeviceConnectionState } from "./useDeviceStore";
import { useEffect } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Event, listen } from "@tauri-apps/api/event";
import { useMutation } from "@tanstack/react-query";
import { useANC, useBatteryLevel, useCharging } from "./useSoundcoreDevice";
import { DeviceSelection } from "../bindings/DeviceSelection";
import { NewTrayDeviceStatus } from "../types/soundcore";

export interface ITrayStatus {
    deviceConnectionState: DeviceConnectionState,
    batteryLevel: DeviceBatteryLevel,
    batteryCharging: DeviceBatteryCharging,
    anc_mode: ANCModes | null,
}

export const useUpdateTray = () => {
    return useMutation({
        mutationFn: (newTray: ITrayStatus) => {
            return updateTrayStatus(newTray);
        }
    })
};

function updateTrayStatus(data: ITrayStatus) {
    let { deviceConnectionState, batteryLevel, batteryCharging, anc_mode } = data;
    let left_batt: BatteryStatus = {
        is_charging: batteryCharging.left,
        battery_level: batteryLevel.left
    }
    let right_batt: BatteryStatus = {
        is_charging: batteryCharging.right,
        battery_level: batteryLevel.right
    }
    let status: NewTrayDeviceStatus = {
        is_connected: deviceConnectionState === DeviceConnectionState.CONNECTED,
        left_status: left_batt,
        right_status: right_batt,
        anc_mode: anc_mode != null ? anc_mode : "NormalMode",
    }
    console.log(status);
    return invoke("set_tray_device_status", { status });
}

export function setTrayMenu(connection_state: DeviceConnectionState) {
    invoke("set_tray_menu", { isConnected: DeviceConnectionState.CONNECTED === connection_state });
}

// export const useUpdateTray = useMutation(async (status: ITrayStatus) => {
//     invoke("set_tray_device_status", { status });
// });

/* https://github.com/tauri-apps/tauri/issues/4630 */
type RemoveListenerBlock = () => void

export function useWindowEvent<Payload>(name: string, callback: (event: Event<Payload>) => void) {
    return useEffect(() => {
        let removeListener: RemoveListenerBlock | undefined

        const setUpListener = async () => {
            removeListener = await listen(name, (event: any) => {
                callback(event as Event<Payload>)
            })
        }

        setUpListener().catch(error => {
            console.error(`Could not set up window event listener. ${error}`)
        })

        return () => {
            removeListener?.()
        }
    }, [name, callback])
}