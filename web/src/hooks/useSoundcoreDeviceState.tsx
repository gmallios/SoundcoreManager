import { useState, useEffect } from "react";
import { ISoundcoreDevice } from "@bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "@soundcore-lib";

export const useSoundcoreDeviceState = (device: ISoundcoreDevice) => {
    const [state, setState] = useState<SoundcoreDeviceState>(device.state.value);
    useEffect(() => {
        if (device.state) {
            const subscription = device.state.subscribe((newValue) => setState(newValue));
            return () => subscription.unsubscribe();
        }
    }, [device.state]);

    return state;
}