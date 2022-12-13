import React, { useEffect, useState } from "react";
import "./App.css";
import A3951InfoCard from "./components/A3951InfoCard";
import EQCard from "./components/EQCard";
import TopBar from "./components/TopBar";
import useDeviceStore, { DeviceConnectionState } from "./hooks/useDeviceStore";
import Stack from '@mui/material/Stack';
import ANCModeCard from "./components/ANCModeCard";
import { getIsConnected, scanForDevices } from "./hooks/useBluetooth";
import DisconnectedScreen from "./components/DisconnectedScreen";
import { ITrayStatus, setTrayMenu, updateTrayStatus, useWindowEvent } from "./hooks/useTray";
import { CircularProgress } from "@mui/material";
import { ANCModes } from "./bindings/ANCModes";



function App() {
  const { sendANCMode, getDeviceStatus, batteryCharging, batteryLevel, getBatteryLevel, getBatteryCharging, connectUUID, deviceConnectionState, getANCMode, deviceStatus, currentANCMode } = useDeviceStore();


  useWindowEvent("anc_sub_change", event => {
    sendANCMode(event.payload as ANCModes);
  });

  useEffect(() => {
    if(selectedDeviceAddr == null){
      
    }
  }, []);

  // May require additional tweaking
  const BATTERY_LEVEL_POLL_RATE = 10000;
  const BATTERY_CHARGING_POLL_RATE = 500;

  const backend_connected = getIsConnected();
  const [selectedDeviceAddr, setSelectedDeviceAddr] = useState(null);
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [isANCFetched, setIsANCFetched] = useState<boolean>(false);
  const [isDeviceStatusFetched, setIsDeviceStatusFetched] = useState<boolean>(false);


  useEffect(() => {
    setTrayMenu(deviceConnectionState);
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
        let trayStatus: ITrayStatus = {
          deviceConnectionState: deviceConnectionState,
          batteryLevel: batteryLevel,
          batteryCharging: batteryCharging,
          anc_mode: currentANCMode,
        }
        updateTrayStatus(trayStatus)
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
      {deviceConnectionState == DeviceConnectionState.CONNECTED ? (
        <Stack>
          <TopBar />
          {isDeviceStatusFetched ? (
            /* TODO: Create a component which wraps all while-connected components */
            <React.Fragment>
              <A3951InfoCard />
              {isANCFetched && <ANCModeCard />}
              <EQCard />
            </React.Fragment>
          ) : (
            <div style={{ width: "100vw", height: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
                <CircularProgress />
            </div>
          )}
        </Stack>
      ) : (
        <DisconnectedScreen />
      )}
    </React.Fragment>
  );
}

export default App;
