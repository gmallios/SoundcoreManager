import { Event, listen } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import { BridgeResponse } from '../types/tauri-backend';

export const useAsyncBridgeEvent = (cb: (event: Event<BridgeResponse>) => void) => {
  return useEffect(() => {
    const unlisten = async () => {
      await listen('async-bridge-event', (event: Event<BridgeResponse>) => {
        cb(event as Event<BridgeResponse>);
      });
    };

    unlisten().catch((error) => {
      console.error(`Could not set up async bridge event listener. ${error}`);
    });

    return () => {
      unlisten();
    };
  }, [cb]);
};
