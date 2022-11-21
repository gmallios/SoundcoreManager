import React from "react";
import { BthScanResult } from "../bindings/ScanResult";
import { List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import BluetoothIcon from '@mui/icons-material/Bluetooth';
import DoneIcon from '@mui/icons-material/Done';

export default function DeviceList({ devices }: { devices: BthScanResult[] }) {

    const [selectedIndex, setSelectedIndex] = React.useState<number>(0);

    return (
        <React.Fragment>
            <List>
                {devices && devices.map((device, index) => (
                    <ListItem disablePadding key={index}>
                        <ListItemButton>
                            <ListItemIcon>
                                <BluetoothIcon />
                            </ListItemIcon>
                            <ListItemText primary={device.name} />
                            {device.is_connected &&
                                <ListItemIcon>
                                    <DoneIcon />
                                </ListItemIcon>
                            }
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </React.Fragment>
    );
}
