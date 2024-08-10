import React from 'react';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import { useShallow } from 'zustand/react/shallow';
import { DiscoveredDevice } from '../types/soundcore-lib';
import { Button, Listbox, ListboxItem, Progress, Spinner } from '@nextui-org/react';
import { ArrowRight, RefreshCcw } from 'lucide-react';
import { BlurredOverlay } from '@components/atoms/BlurredOverlay';

export const BluetoothSearchScreen: React.FC = () => {
  const {
    isLoading: isScanLoading,
    startScan,
    latestScanResults,
    connectDevice,
    connectedAddresses,
    failedConnectionMap,
    removeFailedConnection
  } = useTauriManagerStore(
    useShallow((state) => ({
      isLoading: state.isScanLoading,
      startScan: state.startScan,
      latestScanResults: state.latestScan,
      connectDevice: state.connectDevice,
      connectedAddresses: state.connectedAddresses,
      failedConnectionMap: state.failedConnectionMap,
      removeFailedConnection: state.removeFailedConnection
    }))
  );

  const [selectedDevice, setSelectedDevice] = React.useState<DiscoveredDevice | null>(null);
  const [isConnecting, setIsConnecting] = React.useState(false);

  if (
    isScanLoading &&
    connectedAddresses.size !== 0 &&
    selectedDevice &&
    connectedAddresses.has(selectedDevice.descriptor.addr)
  ) {
    setIsConnecting(false);
  }

  if (selectedDevice && failedConnectionMap.has(selectedDevice.descriptor.addr)) {
    setIsConnecting(false);
    const reason = failedConnectionMap.get(selectedDevice.descriptor.addr);
    // TODO: Show toast or snackbar
    console.error(`Failed to connect to ${selectedDevice.descriptor.name}, reason: ${reason}`);
    removeFailedConnection(selectedDevice.descriptor.addr);
  }

  const connectFabClick = () => {
    if (!selectedDevice) return;
    connectDevice(selectedDevice);
    setIsConnecting(true);
  };

  const searchFabClick = () => {
    startScan();
    setSelectedDevice(null);
  };

  if (isConnecting) {
    return (
      <BlurredOverlay>
        <Spinner size={'lg'} label={'Connecting...'} />
      </BlurredOverlay>
    );
  }

  // useEffect(() => {
  //   startScan();
  // }, []);

  return (
    <div>
      <div className="flex items-center flex-col">
        <div className="w-full flex items-center flex-col gap-2 pt-2">
          <div color="text.secondary">Select a connected device...</div>
          {isScanLoading && <Progress size="sm" isIndeterminate className="w-full" />}
          {!isScanLoading && <div style={{ width: '100vw', height: '0.15rem' }}></div>}
        </div>
        {latestScanResults && (
          <BluetoothDeviceList devices={latestScanResults} setSelectedDevice={setSelectedDevice} />
        )}
        <Button
          isDisabled={!selectedDevice || isConnecting}
          color="primary"
          className="fixed bottom-4 right-4"
          onClick={connectFabClick}>
          Connect <ArrowRight />
        </Button>
        <Button
          isDisabled={isScanLoading || isConnecting}
          className="fixed bottom-4 left-4"
          onClick={searchFabClick}>
          Rescan <RefreshCcw />
        </Button>
      </div>
    </div>
  );
};

const BluetoothDeviceList: React.FC<{
  devices: DiscoveredDevice[];
  setSelectedDevice: (device: DiscoveredDevice) => void;
}> = ({ devices, setSelectedDevice }) => {
  const [selectedItem, setSelectedItem] = React.useState<Set<number>>(new Set([0]));

  const onItemClicked = (item: Set<number>) => {
    setSelectedDevice(devices[item.values().next().value]);
    setSelectedItem(item);
  };

  return (
    <div className="flex w-full">
      <Listbox
        variant="flat"
        disallowEmptySelection
        selectionMode="single"
        selectedKeys={selectedItem}
        onSelectionChange={(keys) => onItemClicked(keys as Set<number>)}>
        {devices.map((device, idx) => (
          <ListboxItem key={idx}>
            {device.descriptor.name.length > 0
              ? device.descriptor.name
              : device.descriptor.addr.address}
          </ListboxItem>
        ))}
      </Listbox>
    </div>
  );
};
