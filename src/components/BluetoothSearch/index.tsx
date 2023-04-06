import { Center, Container, List, Space, Stack, Title, createStyles, rem } from "@mantine/core";
import React, { useEffect, useState } from "react";
import BluetoothItem, { BluetoothItemSkeleton } from "./BluetoothItem";
import { useSearch } from "../../hooks/useBluetooth";
import { NavigationProgress, nprogress } from "@mantine/nprogress";
import { StateSchema, createMachine, interpret } from "xstate";
import { useMachine } from "@xstate/react";
import disconnectedScreenMachine from "./machine";

export function BluetoothSearchScreen() {

    const { status, data } = useSearch();
    const [machineState, sendToMachine] = useMachine(disconnectedScreenMachine, { devTools: true });


    useEffect(() => {
        sendToMachine('SEARCH');
    }, []);

    useEffect(() => {
        if(status === 'success') {
            sendToMachine("RESOLVE");
        } else if(status === 'error') {
            sendToMachine("REJECT");
        }
    }, [status]);


    useEffect(() => {
        if (machineState.matches("searchSuccess")) {
            nprogress.complete();
        } else if(machineState.matches("searching")) {
            nprogress.start();
        }
    }, [machineState]);


    const onItemClicked = (idx: number) => {
        console.log("Connecting to: " + JSON.stringify(data![idx]));
    };

    return (
        <React.Fragment>
            <NavigationProgress />
            <Space h="sm" />
            <Container>
                <Center>
                    <Title order={5}>Select a Soundcore Device to connect to...</Title>
                </Center>
            </Container>
            <Space h="xl" />
            <Stack spacing="md">
                {machineState.matches("searching") &&
                    <>
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                    </>

                }
                {machineState.matches("searchSuccess") && 
                (
                    <>
                        {data!.map((device, idx) => {
                        return (
                            <BluetoothItem key={device.name}
                                idx={idx}
                                name={device.name}
                                isConnected={device.is_connected}
                                model={device.modelid}
                                onItemClicked={(_event, idx) => onItemClicked(idx)}
                            />
                        )
                    })}
                    </>
                )}
            </Stack>
        </React.Fragment>
    )
}