/// <reference types="web-bluetooth" />

import React, { useState } from 'react';
import { generate_soundcore_service_uuids, getSoundcoreMacPrefixes } from '@wasm/manager_wasm';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import { DeviceStateCard } from '@components/DeviceStateCard/deviceStateCard';
import { BLEDeviceFactory } from './ble/bleDevice';
import { Box } from '@mui/material';
import { Button, Navbar, NavbarBrand, NavbarContent, NavbarItem, Spinner } from '@nextui-org/react';
import { EqualizerCard } from '@components/EqualizerCard/equalizerCard';

enum ConnectionDialogStatus {
  DIALOG_OPEN,
  CONNECTING,
  CLOSED
}

export const WebApp: React.FC = () => {
  const { state, device, setDevice } = useWebManagerStore((state) => ({
    state: state.currentState,
    device: state.device,
    setDevice: state.setDevice
  }));
  const [isConnecting, setIsConnecting] = useState(ConnectionDialogStatus.CLOSED);
  const scan = async () => {
    setIsConnecting(ConnectionDialogStatus.DIALOG_OPEN);
    // The serviceUuids must contain the target service for it to be
    // discoverable and connectable. The companyIdentifiers are used
    // to filter the devices in the scan dialog.
    const soundcoreServiceUuids = generate_soundcore_service_uuids();
    const companyIdentifiers = getSoundcoreMacPrefixes();
    try {
      const device = await navigator.bluetooth.requestDevice({
        filters: companyIdentifiers.map((prefix: number[]) => ({
          manufacturerData: [
            {
              companyIdentifier: (prefix[1] << 8) | prefix[0]
            }
          ]
        })),
        optionalServices: soundcoreServiceUuids
      });
      setIsConnecting(ConnectionDialogStatus.CONNECTING);
      setDevice(await BLEDeviceFactory(device));
      setIsConnecting(ConnectionDialogStatus.CLOSED);
    } catch (e) {
      // TODO: Inform the user in some way (toast?)
      setIsConnecting(ConnectionDialogStatus.CLOSED);
    }
  };

  return (
    <Box>
      <Navbar>
        <NavbarBrand>Soundcore Manager</NavbarBrand>
        <NavbarContent justify="end">
          <NavbarItem>
            <Button color="primary" href="#" variant="flat" onClick={scan}>
              Connect to a device
            </Button>
          </NavbarItem>
        </NavbarContent>
      </Navbar>
      <div className={'min-h-fit flex flex-col items-center justify-start p-4'}>
        {state ? (
          <div className={'flex flex-col items-stretch'}>
            <DeviceStateCard state={state} />
            <EqualizerCard state={state} />
          </div>
        ) : (
          <>
            {isConnecting != ConnectionDialogStatus.CLOSED && (
              <div className="fixed inset-0 z-50 flex items-center justify-center bg-opacity-75 backdrop-blur-sm transition-all">
                {isConnecting === ConnectionDialogStatus.CONNECTING && (
                  <Spinner size={'lg'} label={'Connecting...'} />
                )}
              </div>
            )}
          </>
        )}
      </div>
    </Box>
  );
};
