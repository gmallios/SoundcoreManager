import React, { useEffect } from 'react';
import './App.css';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import { BluetoothSearchLayout } from './layouts/bluetoothSearch';
import { useAsyncBridgeEvent, useAsyncBridgeRequest } from '@hooks/useAsyncBridge';
import { useShallow } from 'zustand/react/shallow';
import { DeviceStateLayout } from './layouts/deviceState';

export const TauriApp: React.FC = () => {
  const [isFirstRender, setFirstRender] = React.useState(true);
  const [handleAsyncBridgeEvent, connectedAddresses] = useTauriManagerStore(
    useShallow((state) => [state.handleAsyncBridgeEvent, state.connectedAddresses])
  );

  const currentState = useTauriManagerStore((state) => state.currentViewedDeviceState());

  // Add the event listener to the bridge, which listener is
  // provided by the store.
  useAsyncBridgeEvent((event) => {
    handleAsyncBridgeEvent(event);
  });

  // Disconnect all devices on hard refresh
  useEffect(() => {
    if (isFirstRender) {
      useAsyncBridgeRequest({ command: 'disconnectAll' });
    }
    setFirstRender(false);
  }, []);

  return (
    <React.Fragment>
      {connectedAddresses.size !== 0 && currentState ? (
        <DeviceStateLayout state={currentState} />
      ) : (
        <BluetoothSearchLayout />
      )}
    </React.Fragment>
  );
};

export default TauriApp;
