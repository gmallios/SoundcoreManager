import React, { useEffect, useState } from "react";
import "./App.css";
import { useMachine } from "@xstate/react";
import { assign, createMachine, interpret } from "xstate";
import { BluetoothSearchScreen as SearchScreen } from "./components/Search";
import { HomeScreen } from "./components/Home";
import { BthScanResult } from "./types/tauri-backend";
import useGlobalStore from "./hooks/useGlobalStore";


export type ScreenManagerMachineContext = {
  device: BthScanResult | null;
};
export type ScreenManagerMachineEvent = { type: 'SUCCESS'; device: BthScanResult };
export const screenManagerMachine = createMachine<ScreenManagerMachineContext, ScreenManagerMachineEvent>({
  id: 'screen_manager',
  initial: 'disconnected',
  predictableActionArguments: true,
  context: {
    device: null,
  },
  states: {
    disconnected: {
      on: {
        SUCCESS: {
          target: 'connected',
          actions: (ctx: ScreenManagerMachineContext, event: ScreenManagerMachineEvent) => { ctx.device = event.device }
        },
      }
    },
    connected: {},
  }
});


function App() {
  const [screenManager, _sendScreenManager, screenManagerService] = useMachine(screenManagerMachine, { devTools: true });
  
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
