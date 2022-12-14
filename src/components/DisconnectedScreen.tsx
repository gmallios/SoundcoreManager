import React, { useEffect } from "react";
import { CircularProgress, Fab, Stack, Typography } from "@mui/material";
import { useSearch } from "../hooks/useBluetooth";
import DeviceList from "./DeviceList";
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';
import { BthScanResult } from "../bindings/ScanResult";
import useDeviceStore, { DeviceConnectionState } from "../hooks/useDeviceStore";




export default function DisconnectedScreen() {
    //const { loading, data } = scanForDevices();
    const { isLoading, data } = useSearch();
    const { connectUUID } = useDeviceStore();
    const [selectedDevice, setSelectedDevice] = React.useState<BthScanResult>();
    const { setDeviceConnectionState, close } = useDeviceStore();


    useEffect(() => {
        // setDeviceConnectionState(DeviceConnectionState.DISCONNECTED);
        close();
    }, []);

    const handleFabClick = () => {
        if (selectedDevice) {
            console.log("Connecting to: " + selectedDevice.address)
            connectUUID("A3951", selectedDevice.address);
        }
    };

    if (isLoading) {
        return (
            <div style={{ width: "100vw", height: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
                <CircularProgress />
            </div>
        );
    }

    return (
        <div>
            <Stack sx={{ mb: 2, mt: 2, width: "100vw", display: "flex", alignItems: "center", justifyContent: "center" }}>
                <Typography color="text.secondary">Select a connected device...</Typography>
                <DeviceList sx={{ width: "100vw" }} devices={data} setSelectedDevice={setSelectedDevice} />
                <Fab onClick={() => handleFabClick()} variant="extended" size="medium" color="primary" aria-label="add" sx={{ position: "absolute", bottom: 16, right: 16 }}>
                    Connect
                    <ArrowForwardIcon sx={{ ml: 1 }} />
                </Fab>
            </Stack>
        </div>
    );
}