import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { SoundcoreDeviceState, getUUIDSet, get_state_update_packet } from "../wasm/pkg/soundcore_lib_wasm";
import { connectToSoundcoreDevice } from './bluetooth/SoundcoreBLEConnection';
import { createSoundcoreDevice, selectDevice } from './bluetooth/SoundcoreDevice';

function App() {
  const [count, setCount] = useState(0)



  const a = () => {
    selectDevice("A3951").then((device) => {
      console.log(device.state);
      device.requestNewState();
    });
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
        <button onClick={() => a()}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
