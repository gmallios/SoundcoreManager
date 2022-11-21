import React, { useEffect, useState } from "react";
import "./App.css";
import A3951InfoCard from "./components/A3951InfoCard";
import useDeviceStore, { DeviceConnectionState } from "./hooks/useDeviceStore";
import Stack from '@mui/material/Stack';
import ANCModeCard from "./components/ANCModeCard";
import EQCard from "./components/EQCard";
import { scanForDevices } from "./hooks/useBluetooth";
import DisconnectedScreen from "./components/DisconnectedScreen";

function App() {
  const { getDeviceStatus, tryInitialize, getBatteryLevel, getBatteryCharging, connectUUID, deviceConnectionState, getANCMode, deviceStatus, currentANCMode } = useDeviceStore();


  useEffect(() => {
    // tryInitialize("A3951");
    // connectUUID("AC:12:2F:6A:D2:07", "00001101-0000-1000-8000-00805F9B34FB");
    if(selectedDeviceAddr == null){
      
    }
  }, []);

  // May require additional tweaking
  const BATTERY_LEVEL_POLL_RATE = 10000;
  const BATTERY_CHARGING_POLL_RATE = 500;

  const [selectedDeviceAddr, setSelectedDeviceAddr] = useState(null);
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [isANCFetched, setIsANCFetched] = useState<boolean>(false);
  const [isDeviceStatusFetched, setIsDeviceStatusFetched] = useState<boolean>(false);

  useEffect(() => {
    if (deviceConnectionState == DeviceConnectionState.CONNECTED) {
      getDeviceStatus();
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
    if (deviceStatus != undefined && DeviceConnectionState.CONNECTED) {
      getBatteryCharging();
      getBatteryLevel();
      getANCMode();

      // Poll battery level and charging state at different rates,
      // since the level changes less frequently in comparison to the charging state
      const batteryLevelInterval = setInterval(() => {
        getBatteryLevel();
      }, BATTERY_LEVEL_POLL_RATE);

      const batteryChargingInterval = setInterval(() => {
        getBatteryCharging();
        getDeviceStatus();
      }, BATTERY_CHARGING_POLL_RATE);


      setIsConnected(true);
      setIsDeviceStatusFetched(true);

      return () => {
        // Clear the intervals on unmount
        clearInterval(batteryLevelInterval);
        clearInterval(batteryChargingInterval);
      };

    }
  }, [deviceStatus]);

  return (
    <React.Fragment>
      {deviceConnectionState == DeviceConnectionState.CONNECTED &&
        <Stack>
          <A3951InfoCard />
          {isANCFetched && <ANCModeCard />}
          {isDeviceStatusFetched && <EQCard />}
        </Stack>
      }
      {selectedDeviceAddr == null &&
        <DisconnectedScreen />
      }
    
    </React.Fragment>
  );
}

export default App;
