import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import A3951InfoCard from "./components/A3951InfoCard";
import useDeviceStore, { DeviceConnectionState } from "./hooks/useDeviceStore";


function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");
  const { tryInitialize, getBatteryLevel, connectUUID, deviceConnectionState } = useDeviceStore();

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  useEffect(() => {
    tryInitialize("A3951");
    connectUUID("AC:12:2F:6A:D2:07","00001101-0000-1000-8000-00805F9B34FB");
  }, []);

  useEffect(() => {
    if(deviceConnectionState == DeviceConnectionState.CONNECTED)
      getBatteryLevel();
      
  }, [deviceConnectionState]);

  return (
    <div>
      <A3951InfoCard />
    </div>
  );
}

export default App;
