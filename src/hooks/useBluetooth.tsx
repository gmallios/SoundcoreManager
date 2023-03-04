import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useQuery } from "@tanstack/react-query";
import { BthScanResult } from "../types/tauri-backend";

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

export function useSearch() {
    return useQuery<BthScanResult[], Error>(["bt_search"], async () => {
        const res = await invoke("scan_for_devices");
        return res as BthScanResult[];
    });
}


export function getIsConnected() {
    const [res, setRes] = useState<boolean>(false);
    useEffect(() => {
        const fetchStatus = async () => {
            invoke("is_connected").then((result) => {
                setRes(result as boolean);
            }).catch((err) => {
                setRes(false);
            });
        }
        fetchStatus();
    }, []);
    return { res };
}