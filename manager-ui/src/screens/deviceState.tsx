import { DeviceStateCard } from '@components/DeviceStateCard/deviceStateCard';
import { EqualizerCard } from '@components/EqualizerCard/equalizerCard';
import { SoundModeCard } from '@components/SoundModeCard/soundModeCard';
import { useSoundcoreStore } from '@stores/useSoundcoreStore';
import { useShallow } from 'zustand/react/shallow';

export const DeviceStateScreen = (): JSX.Element => {
  const [connectedAddresses, currentViewedDevice] = useSoundcoreStore(
    useShallow((state) => [state.connectedAddresses, state.currentViewedDevice])
  );

  const currentState = useSoundcoreStore((state) => state.currentViewedDeviceState());

  if (!currentViewedDevice || !currentState) {
    return <h1>No device selected</h1>;
  }
  return (
    <>
      <DeviceStateCard />
      <SoundModeCard state={currentState} />
      {process.env.NODE_ENV === 'development' && (
        <>
          <EqualizerCard state={currentState} />
          <h1>Connected Devices</h1>
          <ul>
            {connectedAddresses.values.map((addr, idx) => (
              <li key={idx}>{addr}</li>
            ))}
          </ul>
          <h1>Device State</h1>
          <ul>{JSON.stringify(currentState)}</ul>\
        </>
      )}
    </>
  );
};
