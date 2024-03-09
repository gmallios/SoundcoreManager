import { Event, listen } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import { BridgeCommand, BridgeResponse } from '../types/tauri-backend';
import { invoke } from '@tauri-apps/api/tauri';

export const AsyncBridgeEvent = 'async-bridge-event';
export const AsyncBridgeRequest = 'send_bridge_command';

export const useAsyncBridgeEvent = (cb: (event: Event<BridgeResponse>) => void) => {
  return useEffect(() => {
    const unlisten = async () => {
      await listen(AsyncBridgeEvent, (event: Event<BridgeResponse>) => {
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

export const useAsyncBridgeRequest = (command: BridgeCommand): Promise<void> => {
  return invoke(AsyncBridgeRequest, { payload: command });
};
