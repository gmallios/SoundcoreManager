import React, { useEffect } from "react";
import { CircularProgress, Fab, Stack, Typography } from "@mui/material";
import { useSearch } from "../hooks/useBluetooth";
import DeviceList from "./DeviceList";
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';
import { BthScanResult } from "../bindings/ScanResult";
import useDeviceStore, { DeviceConnectionState } from "../hooks/useDeviceStore";
import { DeviceSelection } from "../bindings/DeviceSelection";
import { setTrayMenu } from "../hooks/useTray";




export default function DisconnectedScreen() {
    //const { loading, data } = scanForDevices();
    const { isLoading, data } = useSearch();
    const { connectUUID } = useDeviceStore();
    const [selectedDevice, setSelectedDevice] = React.useState<BthScanResult>();
    const { setDeviceConnectionState, close } = useDeviceStore();
    const { updateDeviceModel } = useDeviceStore((state) => ({
        deviceModel: state.deviceModel,
        updateDeviceModel: state.updateDeviceModel,
        shallow: true
    }));

   
    
    useEffect(() => {
        // setDeviceConnectionState(DeviceConnectionState.DISCONNECTED);
        setTrayMenu(DeviceConnectionState.DISCONNECTED);
        close();
        updateDeviceModel("None");
    }, []);

    const handleFabClick = () => {
        if (selectedDevice) {
            let deviceModel: DeviceSelection = selectedDevice.name == "Soundcore Liberty Air 2 Pro" ? "A3951" : "A3027"; /* TODO: Scale this up to multiple modelIds */
            updateDeviceModel(deviceModel);
            console.log("Connecting to: " + selectedDevice.address)
            connectUUID(deviceModel, selectedDevice.address);
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