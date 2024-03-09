import React from 'react';
import './App.css';
import { useSoundcoreStore } from './stores/useSoundcoreStore';
import { BluetoothSearchScreen } from './screens/bluetoothSearch';
import { useAsyncBridgeEvent } from './hooks/useAsyncBridge';
import { useShallow } from 'zustand/react/shallow';

export const App: React.FC = () => {
  const [handleAsyncBridgeEvent, connectedDevices] = useSoundcoreStore(
    useShallow((state) => [state.handleAsyncBridgeEvent, state.connectedDevices])
  );

  // Add the event listener to the bridge, which listener is
  // provided by the store.
  useAsyncBridgeEvent((event) => {
    handleAsyncBridgeEvent(event);
  });

  return (
    <React.Fragment>
      {connectedDevices.length !== 0 ? <h1>Some connected devices!</h1> : <BluetoothSearchScreen />}
    </React.Fragment>
  );
};

export default App;
