import React, { useEffect } from 'react';
import './App.css';
import { useSoundcoreStore } from './stores/useSoundcoreStore';
import { BluetoothSearchScreen } from './screens/bluetoothSearch';
import { useAsyncBridgeEvent, useAsyncBridgeRequest } from './hooks/useAsyncBridge';
import { useShallow } from 'zustand/react/shallow';
import { DeviceStateScreen } from '@screens/deviceState';

export const App: React.FC = () => {
  const [isFirstRender, setFirstRender] = React.useState(true);
  const [handleAsyncBridgeEvent, connectedAddresses] = useSoundcoreStore(
    useShallow((state) => [state.handleAsyncBridgeEvent, state.connectedAddresses])
  );

  const state = useSoundcoreStore((state) => state);
  console.log('state', state);

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
      {connectedAddresses.size !== 0 ? <DeviceStateScreen /> : <BluetoothSearchScreen />}
    </React.Fragment>
  );
};

export default App;
