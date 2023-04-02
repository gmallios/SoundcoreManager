import { Center, Container, List, Stack, Title } from "@mantine/core";
import React, { useState } from "react";
import BluetoothItem from "./BluetoothItem";

export function BluetoothSearchScreen() {

    const [devices, setDevices] = useState([
        { id: 1, name: 'Device 1', description: 'Description for Device 1' },
        { id: 2, name: 'Device 2', description: 'Description for Device 2' },
        { id: 3, name: 'Device 3', description: 'Description for Device 3' },
      ]);
    
      function handleDeviceClick(device) {
        console.log('Clicked on device:', device);
      }
    
    return(
        <React.Fragment>
            <Container>
                <Center>
                    <Title order={5}>Select a Soundcore Device to connect to...</Title>
                </Center>
            </Container>
            <Stack justify="flex-start" align="flex-start" spacing="xs"> 
                <BluetoothItem />
                <BluetoothItem />
                <BluetoothItem />
            </Stack>
        </React.Fragment>
    )
}