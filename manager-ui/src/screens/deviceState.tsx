import { DeviceStateCard } from '@components/DeviceStateCard/deviceStateCard';
import { useSoundcoreStore } from '@stores/useSoundcoreStore';
import { useShallow } from 'zustand/react/shallow';

export const DeviceStateScreen: React.FC = () => {
  const [connectedAddresses, currentViewedDevice] = useSoundcoreStore(
    useShallow((state) => [state.connectedAddresses, state.currentViewedDevice])
  );

  if (!currentViewedDevice) {
    return <h1>No device selected</h1>;
  }

  const state = useSoundcoreStore((state) => state.states).get(currentViewedDevice);

  console.log('state', state);

  return (
    <>
      <DeviceStateCard />
      <h1>Connected Devices</h1>
      <ul>
        {connectedAddresses.values.map((addr, idx) => (
          <li key={idx}>{addr}</li>
        ))}
      </ul>
      <h1>Device State</h1>
      <ul>{JSON.stringify(state)}</ul>
    </>
  );
};
