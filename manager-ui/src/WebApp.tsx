/// <reference types="web-bluetooth" />

import React, { useState } from 'react';
import { generate_soundcore_service_uuids, getSoundcoreMacPrefixes } from '@wasm/manager_wasm';
import { useWebManagerStore } from '@stores/web/useWebManagerStore';
import { BLEDeviceFactory } from './ble/bleDevice';
import {
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Spinner
} from '@nextui-org/react';
import { BlurredOverlay } from '@components/atoms/blurredOverlay';
import { ChevronDown, Unplug } from 'lucide-react';
import { DeviceStateLayout } from './layouts/deviceState';

enum ConnectionDialogStatus {
  DIALOG_OPEN,
  CONNECTING,
  CLOSED
}

export const WebApp: React.FC = () => {
  const { state, disconnect, setDevice } = useWebManagerStore((state) => ({
    state: state.currentState,
    disconnect: state.disconnect,
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
    <>
      <Navbar>
        <NavbarBrand>Soundcore Manager</NavbarBrand>
        <NavbarContent justify="end">
          <NavbarItem>
            {!state ? (
              <Button color="primary" href="#" variant="flat" onClick={scan}>
                Connect to a device
              </Button>
            ) : (
              <Dropdown>
                <DropdownTrigger>
                  <Button variant={'flat'} color={'success'}>
                    Connected <ChevronDown size={20} />
                  </Button>
                </DropdownTrigger>
                <DropdownMenu variant={'faded'} aria-label="Device Actions">
                  <DropdownItem
                    key="disconnect"
                    className="text-danger"
                    color="danger"
                    onClick={disconnect}
                    startContent={
                      <Unplug className={'pointer-events-none flex-shrink-0'} size={20} />
                    }>
                    Disconnect
                  </DropdownItem>
                </DropdownMenu>
              </Dropdown>
            )}
          </NavbarItem>
        </NavbarContent>
      </Navbar>
      <div className={'min-h-fit flex flex-col items-center justify-start p-4'}>
        {state ? (
          <div className={'flex flex-col items-stretch w-2/3 2xl:w-1/2'}>
            <DeviceStateLayout state={state} />
          </div>
        ) : (
          <>
            {isConnecting != ConnectionDialogStatus.CLOSED && (
              <BlurredOverlay>
                {isConnecting === ConnectionDialogStatus.CONNECTING && (
                  <Spinner size={'lg'} label={'Connecting...'} />
                )}
              </BlurredOverlay>
            )}
          </>
        )}
      </div>
    </>
  );
};
