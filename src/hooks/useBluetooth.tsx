import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { BthScanResult } from "../bindings/ScanResult";

export function scanForDevices() {
    const [data, setData] = useState<BthScanResult[]>([]);
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        setLoading(true);
        const fetchDevices = async () => {
            invoke("scan_for_devices").then((result) => {
                let scanResult = result as [BthScanResult];
                setData(scanResult);
                setLoading(false);
            }).catch((err) => {
                setError(err);
                setLoading(false);
            });
        }
        fetchDevices();
    }, []);

    return { data, loading, error };
}