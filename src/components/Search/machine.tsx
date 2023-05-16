import { createMachine } from "xstate";
import { BthScanResult } from "../../types/tauri-backend";
import { invoke } from "@tauri-apps/api";
import useGlobalStore, { DeviceConnectionState } from "../../hooks/useGlobalStore";

export type disconnectedScreenMachineContext = {
    device: BthScanResult | null;
}

export type disconnectedScreenMachineEvent =
    | { type: 'SEARCH' }
    | { type: 'REJECT' }
    | { type: 'RESOLVE' }
    | { type: 'SELECT_DEVICE', device: BthScanResult }
    | { type: 'RETRY' }
    | { type: 'RESOLVE' };

const disconnectedScreenMachine = createMachine<disconnectedScreenMachineContext, disconnectedScreenMachineEvent>(
    {
        id: 'bluetooth_machine',
        initial: 'initial',
        context: {
            device: null
        },
        states: {
            initial: {
                invoke: {
                    id: 'checkIfConnected',
                    src: (_ctx, _event) => closeIfConnected(),
                    onDone: {
                        target: 'searching',
                    },
                    onError: {
                        target: 'searching'
                    },
                },
            },
            searching: {
                after: {
                    10000: 'searchError'
                },
                on: {
                    REJECT: 'searchError',
                    RESOLVE: 'searchSuccess'
                }
            },
            searchSuccess: {
                on: {
                    SELECT_DEVICE: {
                        target: 'connecting',
                        actions: (ctx, event) => { ctx.device = event.device }
                    }
                }
            },
            searchError: {
                on: {
                    RETRY: 'searching'
                }
            },
            connecting: {
                invoke: {
                    id: 'connectToDevice',
                    src: (ctx, event) => connectToDevice(ctx, event),
                    onDone: {
                        target: 'successfullConnection'
                    },
                    onError: {
                        target: 'connectionError'
                    }
                },
            },
            connectionError: {
                on: {
                    RETRY: 'searching' 
                }
            },
            successfullConnection: {
                type: 'final'
            },
        },
    }
);

const connectToDevice = async (ctx: disconnectedScreenMachineContext, _event: disconnectedScreenMachineEvent) => {
    useGlobalStore.getState().setDeviceConnectionState(DeviceConnectionState.CONNECTING);
    await invoke("connect", { btName: ctx.device?.name, btAddr: ctx.device?.address })
        .then((_msg) => {
            console.log("Connected to device:", ctx.device);
            useGlobalStore.getState().setDeviceConnectionState(DeviceConnectionState.CONNECTED);
            return;
        })
        .catch((err) => {
            console.error("Error connecting to device:", err);
            useGlobalStore.getState().setDeviceConnectionState(DeviceConnectionState.DISCONNECTED);
            throw err;
        });
}

const closeIfConnected = async () => {
    await invoke("is_connected").then(async (msg) => {
        console.log("Already connected to device");
        await invoke("close").then((_msg) => {
            console.log("Closed connection");
            useGlobalStore.getState().setDeviceConnectionState(DeviceConnectionState.DISCONNECTED);
        });
    });
}

export default disconnectedScreenMachine;