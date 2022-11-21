import { CircularProgress } from "@mui/material";
import { scanForDevices } from "../hooks/useBluetooth";
import DeviceList from "./DeviceList";

export default function DisconnectedScreen() {
    const { loading, data } = scanForDevices();

    if (loading) {
        return (
            <div style={{ width: "100vw", height: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
                <CircularProgress />
            </div>
        );
    }

    return (
        <div>
            {data &&
                <DeviceList devices={data} />
            }
        </div>
    );
}