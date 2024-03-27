import React from 'react';
import { useSoundcoreStore } from '../stores/useSoundcoreStore';
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
import {
  Button,
  Card,
  CardBody,
  CardHeader,
  Navbar,
  NavbarContent,
  NavbarItem,
  ScrollShadow,
  Skeleton
} from '@nextui-org/react';

export const BluetoothSearchScreen: React.FC = () => {
  const {
    isLoading: isScanLoading,
    startScan,
    latestScanResults,
    connectDevice,
    connectedAddresses,
    failedConnectionMap,
    removeFailedConnection
  } = useSoundcoreStore(
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
    <>
      <h1 className="text-center p-0 pt-3 font-semibold text-small text-foreground">
        Select a device to connect to...
      </h1>

      <Button color="primary" className="fixed bottom-5 right-10 z-50">
        Refresh
      </Button>

      <ScrollShadow className="h-[100vh] overflow-y-auto">
        <div className="flex flex-col gap-5 w-full p-4">
          <DeviceCard />
          <DeviceCard />
          <DeviceCard />
          <DeviceCard />
          <DeviceCard />
          <DeviceCard />
        </div>
      </ScrollShadow>
    </>
  );
  // return (
  //   <div>
  //     {/* <Stack
  //       sx={{
  //         mb: 2,
  //         mt: 2,
  //         width: '100vw',
  //         display: 'flex',
  //         alignItems: 'center',
  //         justifyContent: 'center'
  //       }}> */}
  //     <Container
  //       sx={{
  //         display: 'flex',
  //         alignItems: 'center',
  //         justifyContent: 'center',
  //         flexDirection: 'column',
  //         gap: '0.5rem'
  //       }}>
  //       <Typography color="text.secondary">Select a connected device...</Typography>
  //       {isScanLoading && <LinearProgress sx={{ width: '100vw', height: '0.15rem' }} />}
  //       {!isScanLoading && <div style={{ width: '100vw', height: '0.15rem' }}></div>}
  //     </Container>
  //     {latestScanResults && (
  //       <BluetoothDeviceList devices={latestScanResults} setSelectedDevice={setSelectedDevice} />
  //     )}
  //     <Fab
  //       onClick={() => connectFabClick()}
  //       variant="extended"
  //       size="medium"
  //       color="primary"
  //       aria-label="add"
  //       disabled={!selectedDevice || isConnecting}
  //       sx={{ position: 'absolute', bottom: 16, right: 16 }}>
  //       Connect
  //       <ArrowForwardIcon sx={{ ml: 1 }} />
  //     </Fab>
  //     <Fab
  //       onClick={() => searchFabClick()}
  //       variant="extended"
  //       size="medium"
  //       color="primary"
  //       aria-label="add"
  //       disabled={isScanLoading}
  //       sx={{ position: 'absolute', bottom: 16, left: 16 }}>
  //       Refresh List
  //     </Fab>
  //     {/* </Stack> */}
  //   </div>
  // );
};

const DeviceCard: React.FC = () => {
  return (
    <Card
      className="border-none flex-none bg-background/60 dark:bg-default-100/50 max-w-[610px] last:mb-8"
      shadow="sm">
      <CardBody>
        <div className="flex items-center gap-4">
          <Skeleton className="rounded-lg w-32 h-24">
            <div className="rounded-lg bg-default-300"></div>
          </Skeleton>
          <div className="flex flex-col gap-0">
            <h3 className="font-semibold text-foreground/90">Soundcore Liberty Air 2 Pro</h3>
            <p className="text-small text-foreground/70">A3951</p>
          </div>
        </div>
      </CardBody>
    </Card>
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
