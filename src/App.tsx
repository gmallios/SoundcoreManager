import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import A3951InfoCard from "./components/A3951InfoCard";
import useDeviceStore, { DeviceConnectionState } from "./hooks/useDeviceStore";
import Stack from '@mui/material/Stack';
import ANCModeCard from "./components/ANCModeCard";
import EQCard from "./components/EQCard";

function App() {
  const { currentANCMode, tryInitialize, getBatteryLevel, getBatteryCharging, connectUUID, deviceConnectionState, getANCMode } = useDeviceStore();



  useEffect(() => {
    tryInitialize("A3951");
    connectUUID("AC:12:2F:6A:D2:07", "00001101-0000-1000-8000-00805F9B34FB");
  }, []);

  // May require additional tweaking
  const BATTERY_LEVEL_POLL_RATE = 10000;
  const BATTERY_CHARGING_POLL_RATE = 500;

  useEffect(() => {
    if (deviceConnectionState == DeviceConnectionState.CONNECTED) {
      // Initializes the state
      getBatteryCharging();
      getBatteryLevel();
      getANCMode();

      const batteryLevelInterval = setInterval(() => {
        getBatteryLevel();
      }, BATTERY_LEVEL_POLL_RATE);

      const batteryChargingInterval = setInterval(() => {
        getBatteryCharging();
      }, BATTERY_CHARGING_POLL_RATE);

      return () => {
        clearInterval(batteryLevelInterval);
        clearInterval(batteryChargingInterval);
      };
    }

  }, [deviceConnectionState]);

  return (
    <div>
      {deviceConnectionState == DeviceConnectionState.CONNECTED &&
        <Stack>
          <A3951InfoCard />
          {currentANCMode != null && <ANCModeCard /> }
          <EQCard />
        </Stack>
      }
    </div>
  );
}

export default App;
