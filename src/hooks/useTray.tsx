import { invoke } from "@tauri-apps/api/tauri";
import { DeviceConnectionState } from "./useDeviceStore";
import { useEffect } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Event, listen } from "@tauri-apps/api/event";
import { useMutation } from "@tanstack/react-query";
import { useANC, useBatteryLevel, useCharging } from "./useSoundcoreDevice";
import { ANCModes, NewTrayDeviceStatus } from "../types/tauri-backend";
import { BatteryCharging, BatteryLevel } from "../types/soundcore-lib";

export interface ITrayStatus {
    deviceConnectionState: DeviceConnectionState,
    level: BatteryLevel,
    charging: BatteryCharging,
    anc_mode: ANCModes,
}

export const useUpdateTray = () => {
    return useMutation({
        mutationFn: (newTray: ITrayStatus) => {
            return updateTrayStatus(newTray);
        }
    })
};

function updateTrayStatus(data: ITrayStatus) {
    let { deviceConnectionState, level, charging, anc_mode } = data;
    let status: NewTrayDeviceStatus = {
        is_connected: deviceConnectionState === DeviceConnectionState.CONNECTED,
        level,
        charging,
        anc_mode
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