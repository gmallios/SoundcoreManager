import { invoke } from "@tauri-apps/api";
import { SoundcoreRequestMessage } from "../types/tauri-backend";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

const emitSoundcoreEvent = (payload: SoundcoreRequestMessage) => {
    invoke("soundcore_command", payload);
}

await listen("soundcore_command", (event) => {
    console.log(event);
});