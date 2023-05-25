import { Center, Container, List, Loader, Space, Stack, Title, Transition, createStyles, rem } from "@mantine/core";
import React, { useEffect, useState, useContext, FC } from "react";
import BluetoothItem, { BluetoothItemSkeleton } from "./BluetoothItem";
import { useSearch } from "../../hooks/useBluetooth";
import { NavigationProgress, nprogress } from "@mantine/nprogress";
import { Interpreter, InterpreterFrom, StateFrom, StateSchema, createMachine, interpret } from "xstate";
import { useActor, useMachine } from "@xstate/react";
import disconnectedScreenMachine from "./machine";
import { screenManagerMachine } from "../../App";
import { BthScanResult } from "../../types/tauri-backend";
import { IconMoodSad } from "@tabler/icons-react";
import useGlobalStore from "../../hooks/useGlobalStore";
import { shallow } from "zustand/shallow";


export const BluetoothSearchScreen: FC<{
    screenService: InterpreterFrom<typeof screenManagerMachine>;
}> = ({ screenService }) => {
    const [state, send, service] = useMachine(disconnectedScreenMachine, { devTools: true });

    return <BluetoothSearchView service={service} state={state} screenService={screenService} />

}

const BluetoothSearchView: FC<{
    service: InterpreterFrom<typeof disconnectedScreenMachine>;
    state: StateFrom<typeof disconnectedScreenMachine>;
    screenService: InterpreterFrom<typeof screenManagerMachine>;
}> = ({ service, state, screenService }) => {

    const { status, data } = useSearch();
    const [device, setDevice] = useState<BthScanResult | null>(null);
    const [showDevices, setShowDevices] = useState<boolean>(false);
    const transitionPeriod = 200;
    const { setBtDevice } = useGlobalStore((state) => ({ setBtDevice: state.setBtDevice }), shallow);

    useEffect(() => {
        service.send('SEARCH');
    }, []);

    useEffect(() => {
        if (state.matches("searching")) {
            if (status === 'success') {
                service.send("RESOLVE");
                setShowDevices(true);
            } else if (status === 'error') {
                service.send("REJECT");
                setShowDevices(false);
            }
        }
    }, [status]);


    useEffect(() => {
        if (state.matches("searchSuccess")) {
            nprogress.complete();
            setShowDevices(true);
        } else if (state.matches("searching")) {
            nprogress.start();
            setShowDevices(false);
        } else if (state.matches("successfullConnection")) {
            screenService.send({ type: "SUCCESS", device: device! });
        }
    }, [state]);


    const sleep = (ms: number | undefined) =>
        new Promise(resolve => setTimeout(resolve, ms));

    const onItemClicked = (idx: number) => {
        setDevice(data![idx]);
        setBtDevice(data![idx]);
        setShowDevices(false);
        sleep(transitionPeriod).then(() => {
            service.send({ type: "SELECT_DEVICE", device: data![idx] });
        });
    };

    return (
        <>
            {state.matches("searching") &&
                <>
                    <NavigationProgress />
                    <Space h="sm" />
                    <Space h="xl" />
                    <Stack spacing="md">
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                    </Stack>
                </>
            }
            {state.matches("searchSuccess") &&
                (
                    <>
                        <Space h="sm" />
                        <Container>
                            <Center>
                                <Transition
                                    mounted={showDevices}
                                    transition="fade" duration={transitionPeriod}
                                    timingFunction="ease-in-out">
                                    {(styles) => <Title order={5} style={styles}>Select a Soundcore Device to connect to...</Title>}
                                </Transition>
                            </Center>
                        </Container>
                        <Space h="xl" />
                        <Stack spacing="md">
                            {data!.map((device, idx) => {
                                return (
                                    <Transition
                                        key={device.name}
                                        mounted={showDevices}
                                        transition="fade" duration={transitionPeriod}
                                        timingFunction="ease-in-out">
                                        {(styles) => (
                                            <BluetoothItem
                                                key={device.name}
                                                idx={idx}
                                                name={device.name}
                                                isConnected={device.is_connected}
                                                model={device.modelid}
                                                onItemClicked={(_event, idx) => onItemClicked(idx)}
                                                styles={styles}
                                            />
                                        )}
                                    </Transition>
                                )
                            })}
                        </Stack>
                    </>
                )}
            {state.matches("connecting") &&
                <Transition
                    mounted={true}
                    transition="fade" duration={transitionPeriod}
                    timingFunction="ease-in-out">
                    {(styles) => (
                        <div style={{ width: "100vw", height: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
                            <Loader color="red" size="xl" variant="dots" style={styles} />
                        </div>
                    )}
                </Transition>
            }
            {state.matches("searchError") &&
                <div style={{ width: "100vw", height: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
                    <IconMoodSad />
                </div>
            }
        </>
    )
}