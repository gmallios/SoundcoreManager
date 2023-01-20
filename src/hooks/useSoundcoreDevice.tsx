/* Not in use - Draft */
/* Move to async state react-query and define here async functions to "fetch" */
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { ANCModes } from "../bindings/ANCModes";
import useDeviceStore, { DeviceBatteryCharging, DeviceBatteryLevel, DeviceConnectionState, DeviceStatus, EQWave } from "./useDeviceStore";



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
    const { deviceConnectionState } = useDeviceStore();
    return useQuery<DeviceBatteryCharging, Error>(["charging"], async () => {
        try {
            const result = await invoke("get_battery_charging");
            return result as DeviceBatteryCharging;
        } catch (err) {
            console.error("Charging Error: " + err);
            return { left: false, right: false } as DeviceBatteryCharging;
        }
    }, {
        refetchInterval: 500,
        enabled: deviceConnectionState == DeviceConnectionState.CONNECTED,
    });
}

export function useBatteryLevel() {
    const { deviceConnectionState } = useDeviceStore();
    return useQuery<DeviceBatteryLevel, Error>(["battery"], async () => {
        try {
            const result = await invoke("get_battery_level");
            return result as DeviceBatteryLevel;
        } catch (err) {
            console.error("Battery Error: " + err);
            return { left: 0, right: 0 } as DeviceBatteryLevel;
        }
    }, {
        refetchInterval: 5000,
        enabled: deviceConnectionState == DeviceConnectionState.CONNECTED,
    });
}

export function useStatus() {
    const { deviceConnectionState } = useDeviceStore();
    return useQuery<DeviceStatus, Error>(["status"], async () => {
        const result = await invoke("get_status");
        return result as DeviceStatus;
    }, {
        refetchInterval: 5000,
        enabled: deviceConnectionState == DeviceConnectionState.CONNECTED,
    });
}

export function useANC() {
    const { deviceConnectionState } = useDeviceStore();
    return useQuery<ANCModes, Error>(["anc"], async () => {
        const result = await invoke("get_anc");
        return result as ANCModes;
    }, {
        refetchInterval: 2000,
        enabled: deviceConnectionState == DeviceConnectionState.CONNECTED,
    });
}

export function useUpdateANC() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: (newMode: ANCModes) => { return invoke("set_anc", { mode: newMode }); },
        onMutate: async (newMode: ANCModes) => {
            await queryClient.cancelQueries({ queryKey: ["anc"] });
            queryClient.setQueryData<ANCModes>(["anc"], newMode);
        },
    });
}

export function useUpdateEQ() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: (newEQ: EQWave) => { return invoke("set_eq", { eq: newEQ }); },
        onMutate: async (newEQ: EQWave) => {
            await queryClient.cancelQueries({ queryKey: ["status"] });
            queryClient.setQueryData<DeviceStatus>(["status"], old => {
                return {
                    ...old!,
                    left_eq: newEQ,
                    right_eq: newEQ,
                }
            });
        },
    });
}