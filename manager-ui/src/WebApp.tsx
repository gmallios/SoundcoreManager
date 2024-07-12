/// <reference types="web-bluetooth" />

import React, { useState } from 'react';
import {
  generate_soundcore_service_uuids,
  getSoundcoreMacPrefixes,
  WebBLEDevice
} from '@wasm/manager_wasm';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import { DeviceStateCard } from '@components/DeviceStateCard/deviceStateCard';
import { SoundModeCard } from '@components/SoundModeCard/soundModeCard';
import { BLEDevice } from './ble/bleDevice';
import { Box } from '@mui/material';
import { EqualizerCard } from '@components/EqualizerCard/equalizerCard';

export const WebApp: React.FC = () => {
  const state = useWebManagerStore((state) => state.currentState);
  const setDevice = useWebManagerStore((state) => state.setDevice);
  const [isConnecting, setIsConnecting] = useState(false);
  const scan = async () => {
    // The serviceUuids must contain the target service for it to be
    // discoverable and connectable. The companyIdentifiers are used
    // to filter the devices in the scan dialog.
    const soundcoreServiceUuids = generate_soundcore_service_uuids();
    const companyIdentifiers = getSoundcoreMacPrefixes();
    try {
      const device = await navigator.bluetooth.requestDevice({
        filters: companyIdentifiers.map((prefix) => ({
          manufacturerData: [
            {
              companyIdentifier: (prefix[1] << 8) | prefix[0]
            }
          ]
        })),
        optionalServices: soundcoreServiceUuids
      });
      setIsConnecting(true);
      setDevice(new BLEDevice(await WebBLEDevice.new(device)));
      setIsConnecting(false);
    } catch (e) {
      setIsConnecting(false);
    }
  };

  return (
    <Box>
      {state ? (
        <>
          <DeviceStateCard state={state} />
          <SoundModeCard state={state} />
          <EqualizerCard state={state} />
        </>
      ) : (
        <>
          <button onClick={scan}>Connect</button>
          {isConnecting && <a>Connecting...</a>}
        </>
      )}
    </Box>
  );
};
