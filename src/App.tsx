import React, { useEffect } from 'react';
import './App.css';
import OverviewCard from './components/OverviewCard';
import EQCard from './components/EQCard';
import useDeviceStore, { DeviceConnectionState } from './hooks/useDeviceStore';
import Stack from '@mui/material/Stack';
import ANCModeCard from './components/ANCModeCard/ANCModeCard';
import DisconnectedScreen from './components/DisconnectedScreen';
import { ITrayStatus, useUpdateTray, useWindowEvent } from './hooks/useTray';
import { CircularProgress } from '@mui/material';
import {
  useANC,
  useBatteryLevel,
  useCharging,
  useStatus,
  useUpdateANC
} from './hooks/useSoundcoreDevice';
import { ANCModes } from './types/tauri-backend';
import { useSoundcoreStore } from './stores/useSoundcoreStore';

function App() {
  const { deviceConnectionState } = useDeviceStore();

  const { data: level, isSuccess: isBatteryLevelSuccess } = useBatteryLevel();
  const { data: charging, isSuccess: isBatteryChargingSuccess } = useCharging();
  const { data: ancStatus, isSuccess: isANCStatusSuccess } = useANC();
  const { data: devStatus, isSuccess: isStatusSuccess } = useStatus();
  const isDataSuccess =
    isBatteryLevelSuccess && isBatteryChargingSuccess && isANCStatusSuccess && isStatusSuccess;
  const isDataNotNull =
    level != undefined && charging != undefined && ancStatus != undefined && devStatus != undefined;
  const trayMutation = useUpdateTray();
  const ancMutation = useUpdateANC();

  const deviceStates = useSoundcoreStore((state) => state.states);

  console.log('Device state: ', deviceStates);

  /* On Tray Event - Handles the anc submenu event */
  useWindowEvent('anc_sub_change', (event) => {
    ancMutation.mutate(event.payload as ANCModes);
  });

  /* Update tray status on every change */
  useEffect(() => {
    if (
      deviceConnectionState == DeviceConnectionState.CONNECTED &&
      isDataSuccess &&
      isDataNotNull
    ) {
      const trayStatus: ITrayStatus = {
        deviceConnectionState: deviceConnectionState,
        level,
        charging,
        anc_mode: ancStatus
      };
      trayMutation.mutate(trayStatus);
    }
  }, [level, charging, ancStatus, deviceConnectionState]);

  // useEffect(() => {
  //   console.log("Device connection state changed to: " + deviceConnectionState);
  //   setTrayMenu(deviceConnectionState);
  // }, [deviceConnectionState]);

  return (
    <React.Fragment>
      {deviceConnectionState != DeviceConnectionState.DISCONNECTED ? (
        <Stack>
          {isStatusSuccess ? (
            <React.Fragment>
              <OverviewCard />
              <ANCModeCard />
              <EQCard />
            </React.Fragment>
          ) : (
            <div
              style={{
                width: '100vw',
                height: '100vh',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center'
              }}>
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
