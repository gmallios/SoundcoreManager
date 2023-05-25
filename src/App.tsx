import React, { FC, useEffect, useState } from "react";
import "./App.css";
import { useMachine } from "@xstate/react";
import { assign, createMachine, interpret } from "xstate";
import { BluetoothSearchScreen } from "./components/Search";
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


const App: FC = () => {
  const [screenManager, _sendScreenManager, screenManagerService] = useMachine(screenManagerMachine, { devTools: true });
  
  return (
    <>
      {screenManager.matches('disconnected') && <BluetoothSearchScreen screenService={screenManagerService} />}
      {screenManager.matches('connected') && <HomeScreen />}
    </>
  )
}

export default App;
