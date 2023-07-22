import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { ISoundcoreDevice, selectDevice } from '@bluetooth/SoundcoreDevice';
import { useSoundcoreDeviceState } from '@hooks/useSoundcoreDeviceState';

function App() {
  const [device, setDevice] = useState<ISoundcoreDevice>();

  const connect = () => {
    selectDevice().then((device) => {
      setDevice(device);
    });
  }

  const disconnect = () => {
    if (device) {
      device.disconnect();
      setDevice(undefined);
    }
  }
  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => connect()}>
          Connect
        </button>
        <button onClick={() => disconnect()}>
          Disconnect
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      {device && <StateComponent device={device} />}
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

function StateComponent({ device }: { device: ISoundcoreDevice }) {
  const actualState = useSoundcoreDeviceState(device);

  useEffect(() => {
    console.log("New state: ", actualState);
  }, [actualState]);

  return (
    <div className="card">
      <button onClick={() => device.requestNewState()}>Request New State</button>
      <p>{actualState.battery_level.left}</p>
    </div>
  );
}



export default App
