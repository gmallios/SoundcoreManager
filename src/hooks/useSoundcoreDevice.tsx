/* Not in use - Draft */
/* Move to async state react-query and define here async functions to "fetch" */
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { DeviceBatteryCharging, DeviceBatteryLevel, DeviceConnectionState } from "./useDeviceStore";

export enum SupportedModelIDs {
    A3951 = "A3951",
}

export function tryInitialize(modelID: SupportedModelIDs) {
    invoke("init_device", { device: modelID });
}

export function connectWithUUID(macAddr: String, uuid: String) {
    const [status, setStatus] = useState<DeviceConnectionState>(DeviceConnectionState.UNINITIALIZED); 
    const [connecting, setConnecting] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        setConnecting(true);
        setStatus(DeviceConnectionState.CONNECTING);
        const connect = async () => {
            invoke("connect_device", { macAddr: macAddr, uuid: uuid }).then((_result) => {
                setStatus(DeviceConnectionState.CONNECTED);
                setConnecting(false);
            }).catch((err) => {
                setStatus(DeviceConnectionState.DISCONNECTED);
                setError(err);
                setConnecting(false);
            });
        }
        connect();
    }, []);

    return { status, connecting, error };
}

export function useCharging() {
    return useQuery<DeviceBatteryCharging, Error>(["charging"], async () => {
        const result = await invoke("get_battery_charging");
        return result as DeviceBatteryCharging;
    }, {
        refetchInterval: 650,
        cacheTime: 650,
    });
}

export function useBatteryLevel() {
    return useQuery<DeviceBatteryLevel, Error>(["battery"], async () => {
        const result = await invoke("get_battery_level");
        return result as DeviceBatteryLevel;
    }, {
        refetchInterval: 2000,
        cacheTime: 2000,
    });
}