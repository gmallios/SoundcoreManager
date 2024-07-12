import React from 'react';
import { useTauriManagerStore } from '@stores/tauri/useTauriManagerStore';
import Stack from '@mui/material/Stack/Stack';
import {
  Container,
  Fab,
  LinearProgress,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Typography
} from '@mui/material';
import { useShallow } from 'zustand/react/shallow';
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';
import { DiscoveredDevice } from '../types/soundcore-lib';
import BluetoothIcon from '@mui/icons-material/Bluetooth';

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
      <div>
        <Stack
          sx={{
            mb: 2,
            mt: 2,
            width: '100vw',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }}>
          <Container
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              flexDirection: 'column',
              gap: '0.5rem'
            }}>
            <Typography color="text.secondary">Connecting...</Typography>
            <LinearProgress sx={{ width: '100vw', height: '0.15rem' }} />
          </Container>
        </Stack>
      </div>
    );
  }

  // useEffect(() => {
  //   startScan();
  // }, []);

  return (
    <div>
      <Stack
        sx={{
          mb: 2,
          mt: 2,
          width: '100vw',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center'
        }}>
        <Container
          sx={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexDirection: 'column',
            gap: '0.5rem'
          }}>
          <Typography color="text.secondary">Select a connected device...</Typography>
          {isScanLoading && <LinearProgress sx={{ width: '100vw', height: '0.15rem' }} />}
          {!isScanLoading && <div style={{ width: '100vw', height: '0.15rem' }}></div>}
        </Container>
        {latestScanResults && (
          <BluetoothDeviceList devices={latestScanResults} setSelectedDevice={setSelectedDevice} />
        )}
        <Fab
          onClick={() => connectFabClick()}
          variant="extended"
          size="medium"
          color="primary"
          aria-label="add"
          disabled={!selectedDevice || isConnecting}
          sx={{ position: 'absolute', bottom: 16, right: 16 }}>
          Connect
          <ArrowForwardIcon sx={{ ml: 1 }} />
        </Fab>
        <Fab
          onClick={() => searchFabClick()}
          variant="extended"
          size="medium"
          color="primary"
          aria-label="add"
          disabled={isScanLoading}
          sx={{ position: 'absolute', bottom: 16, left: 16 }}>
          Refresh List
        </Fab>
      </Stack>
    </div>
  );
};

const BluetoothDeviceList: React.FC<{
  devices: DiscoveredDevice[];
  setSelectedDevice: (device: DiscoveredDevice) => void;
}> = ({ devices, setSelectedDevice }) => {
  const [selectedIndex, setSelectedIndex] = React.useState<number>();

  const onItemClicked = (idx: number) => {
    setSelectedDevice(devices[idx]);
    setSelectedIndex(idx);
  };

  return (
    <List sx={{ width: '100vw' }}>
      {devices.map((device, idx) => (
        <BluetoothDeviceListItem
          key={idx}
          idx={idx}
          device={device}
          onClick={onItemClicked}
          selected={selectedIndex === idx}
        />
      ))}
    </List>
  );
};

const BluetoothDeviceListItem: React.FC<{
  idx: number;
  device: DiscoveredDevice;
  onClick: (idx: number) => void;
  selected?: boolean;
}> = ({ idx, device, onClick, selected }) => {
  return (
    <ListItem disablePadding>
      <ListItemButton selected={selected} onClick={() => onClick(idx)}>
        <ListItemIcon>
          <BluetoothIcon />
        </ListItemIcon>
        <ListItemText primary={device.descriptor.name} />
      </ListItemButton>
    </ListItem>
  );
};
