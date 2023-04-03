import { Center, Container, List, Space, Stack, Title, createStyles, rem } from "@mantine/core";
import React, { useEffect, useState } from "react";
import BluetoothItem, { BluetoothItemSkeleton } from "./BluetoothItem";
import { useSearch } from "../../hooks/useBluetooth";

export function BluetoothSearchScreen() {

    const { isLoading, data } = useSearch();


    useEffect(() => {
        console.log(data)
    }, [data]);

    const [exampleDevices, setExampleDevices] = useState([{
        idx: 0,
        name: "Soundcore Liberty Air 2 Pro",
        isConnected: true,
        isSelected: false,
        model: "A3027"
    }, {
        idx: 1,
        name: "Soundcore Liberty Air 2 Pro",
        isConnected: false,
        isSelected: false,
        model: "A3040"
    }, {
        idx: 2,
        name: "Soundcore Liberty Air 2 Pro",
        isConnected: false,
        isSelected: true,
        model: "A3951"
    }]);


    const onItemClicked = (idx: number) => {
        const newDevices = exampleDevices.map((device) => {
            device.isSelected = device.idx === idx;
            return device;
        });
        setExampleDevices(newDevices);
    }

    return (
        <React.Fragment>
            <Space h="sm" />
            <Container>
                <Center>
                    <Title order={5}>Select a Soundcore Device to connect to...</Title>
                </Center>
            </Container>
            <Space h="xl" />
            <Stack spacing="md">
                {isLoading &&
                    <>
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                        <BluetoothItemSkeleton />
                    </>

                }
                {!isLoading && data!.map((device, idx) => {
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
            </Stack>
        </React.Fragment>
    )
}