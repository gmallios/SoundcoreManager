import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import A3951InfoCard from "./components/A3951InfoCard";
import useDeviceStore, { DeviceConnectionState } from "./hooks/useDeviceStore";
import Stack from '@mui/material/Stack';
import ANCModeCard from "./components/ANCModeCard";
import EQCard from "./components/EQCard";

function App() {
  const { getDeviceStatus, tryInitialize, getBatteryLevel, getBatteryCharging, connectUUID, deviceConnectionState, getANCMode, deviceStatus, currentANCMode } = useDeviceStore();



  useEffect(() => {
    tryInitialize("A3951");
    connectUUID("AC:12:2F:6A:D2:07", "00001101-0000-1000-8000-00805F9B34FB");
  }, []);

  // May require additional tweaking
  const BATTERY_LEVEL_POLL_RATE = 10000;
  const BATTERY_CHARGING_POLL_RATE = 500;

  const [isConnected, setIsConnected] = useState(false);
  const [isANCFetched, setIsANCFetched] = useState(false);
  const [isDeviceStatusFetched, setIsDeviceStatusFetched] = useState(false);

  useEffect(() => {
    if (deviceConnectionState == DeviceConnectionState.CONNECTED) {
      // Initializes the state
      getDeviceStatus();
      getBatteryCharging();
      getBatteryLevel();
      getANCMode();
      setIsConnected(true);


      // Poll battery level and charging state at different rates,
      // since the level changes less frequently in comparison to the charging state
      const batteryLevelInterval = setInterval(() => {
        getBatteryLevel();
      }, BATTERY_LEVEL_POLL_RATE);

      const batteryChargingInterval = setInterval(() => {
        getBatteryCharging();
      }, BATTERY_CHARGING_POLL_RATE);

      return () => {
        // Clear the intervals on unmount
        clearInterval(batteryLevelInterval);
        clearInterval(batteryChargingInterval);
      };
    } else if (deviceConnectionState == DeviceConnectionState.DISCONNECTED) {
      setIsConnected(false);
    }
  }, [deviceConnectionState]);


  useEffect(() => {
    if (currentANCMode != undefined) {
      setIsANCFetched(true);
    }
  }, [currentANCMode]); 

  useEffect(() => {
    if (deviceStatus != undefined) {
      setIsDeviceStatusFetched(true);
      console.log(deviceStatus)
    }
  }, [deviceStatus]);

  return (
    <div>
      {deviceConnectionState == DeviceConnectionState.CONNECTED &&
        <Stack>
          <A3951InfoCard />
          {isANCFetched && <ANCModeCard /> }
          {isDeviceStatusFetched &&  <EQCard />} 
        </Stack>
      }
    </div>
  );
}

export default App;
