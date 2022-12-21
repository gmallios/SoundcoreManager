import React, { useEffect } from "react";
import { BthScanResult } from "../bindings/ScanResult";
import { List, SxProps, } from "@mui/material";
import DeviceListItem from "./DeviceListItem";


interface IDeviceListProps {
    devices: BthScanResult[] | undefined;
    sx: SxProps;
    setSelectedDevice: (device: BthScanResult) => void;
}

export default function DeviceList(props: IDeviceListProps) {
    const { devices, sx, setSelectedDevice } = props;

    if (devices) {
        const connectedDevices = devices.filter(device => device.is_connected);
        const [selectedIndex, setSelectedIndex] = React.useState<number>(0);

        useEffect(() => {
            setSelectedDevice(connectedDevices[selectedIndex]);
        }, [selectedIndex]);

        return (
            <React.Fragment>
                <List sx={sx}>
                    {devices && connectedDevices.map((device, idx) => (
                        <DeviceListItem
                            key={idx}
                            idx={idx}
                            name={device.name}
                            isConnected={device.is_connected}
                            isSelected={selectedIndex === idx}
                            onItemClicked={(_event, idx) => { setSelectedIndex(idx); }} />
                    ))}
                </List>
            </React.Fragment>
        );
    } else {
        return <React.Fragment></React.Fragment>;
    }
}
