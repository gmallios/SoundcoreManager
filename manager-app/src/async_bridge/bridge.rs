use std::sync::Arc;

use log::debug;
use tauri::AppHandle;
use tokio::sync::{mpsc, Mutex};

use soundcore_lib::device::SoundcoreBLEDevice;
use soundcore_lib::{
    ble::BLEConnectionManager,
    device_manager::{create_device_manager, DeviceManager},
};

use super::{BridgeCommand, BridgeResponse, NewStateResponse};

struct CommandLoopState<B: BLEConnectionManager> {
    manager: DeviceManager<B>,
}

impl<B: BLEConnectionManager> CommandLoopState<B> {
    fn new(manager: DeviceManager<B>) -> Self {
        Self {
            manager,
        }
    }
}

pub async fn async_bridge(
    mut input_rx: mpsc::Receiver<BridgeCommand>,
    output_tx: mpsc::Sender<BridgeResponse>,
) {
    tokio::spawn(async move {
        let manager =
            create_device_manager().await;

        // Adapter events
        let mut manager_event_channel = manager.get_event_channel().await.unwrap();
        let adapter_tx = output_tx.clone();
        tokio::task::spawn(async move {
            while let Some(event) = manager_event_channel.recv().await {
                adapter_tx
                    .send(BridgeResponse::AdapterEvent(event))
                    .await
                    .expect("Failed to send adapter event");
            }
        });

        // Main command loop
        let command_loop_state = Arc::new(Mutex::new(CommandLoopState::new(manager)));
        loop {
            while let Some(command) = input_rx.recv().await {
                let command_loop_state = command_loop_state.clone();
                let response = handle_command(command_loop_state, command, output_tx.clone()).await;
                output_tx
                    .send(response)
                    .await
                    .expect("Failed to send response");
            }
        }
    });
}

async fn handle_command<B: BLEConnectionManager>(
    command_loop_state: Arc<Mutex<CommandLoopState<B>>>,
    command: BridgeCommand,
    output_tx: mpsc::Sender<BridgeResponse>,
) -> BridgeResponse {
    match command {
        BridgeCommand::Scan => command_loop_state
            .lock()
            .await
            .manager
            .ble_scan(None)
            .await
            .map(BridgeResponse::ScanResult),
        BridgeCommand::Disconnect(addr) => {
            let addr_clone = addr.clone();
            command_loop_state
                .lock()
                .await
                .manager
                .disconnect(addr)
                .await
                .map(|_| BridgeResponse::Disconnected(addr_clone))
        }
        BridgeCommand::Connect(d) => {
            let addr = d.clone().descriptor.addr;
            let device = command_loop_state.lock().await.manager.connect(d).await;
            let addr_clone = addr.clone();
            if let Ok(device) = device {
                // Get the state channel and listen for changes in the background
                let mut state_channel = device.state_channel().await;
                tokio::task::spawn(async move {
                    while let Ok(()) = state_channel.changed().await {
                        let state = state_channel.borrow().clone();
                        // TODO: Add logging
                        let new_state_response = NewStateResponse {
                            addr: addr_clone.clone(),
                            state,
                        };
                        output_tx
                            .send(BridgeResponse::NewState(new_state_response))
                            .await;
                    }
                });
                Ok(BridgeResponse::ConnectionEstablished(addr))
            } else {
                Err(device.err().unwrap())
            }
        }
    }
    .map_err(|e| BridgeResponse::Error(e.to_string()))
    .unwrap_or_else(|e| e)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     async fn create_bridge() -> (mpsc::Sender<BridgeCommand>, mpsc::Receiver<BridgeResponse>) {
//         let (input_tx, input_rx) = mpsc::channel(1);
//         let (output_tx, output_rx) = mpsc::channel(1);
//         async_bridge(input_rx, output_tx).await;
//         (input_tx, output_rx)
//     }
//
//     #[tokio::test]
//     async fn should_handle_scan_command_and_produce_response() {
//         let (input_tx, mut output_rx) = create_bridge().await;
//         input_tx
//             .send(BridgeCommand::ScanBle)
//             .await
//             .expect("Failed to send command");
//
//         let response = output_rx.recv().await.expect("Failed to receive response");
//
//         match response {
//             BridgeResponse::ScanResult(res) => {
//                 assert!(!res.is_empty());
//             }
//             _ => panic!("Unexpected response: {:?}", response),
//         }
//     }
//
//     #[tokio::test]
//     async fn should_handle_connect_command_and_produce_response() {
//         let (input_tx, mut output_rx) = create_bridge().await;
//         input_tx
//             .send(BridgeCommand::ScanBle)
//             .await
//             .expect("Failed to send command");
//
//         let scan_response = output_rx.recv().await.expect("Failed to receive response");
//
//         let devices = match scan_response {
//             BridgeResponse::ScanResult(res) => res,
//             _ => panic!("Unexpected response: {:?}", scan_response),
//         };
//
//         let device = devices.first().unwrap();
//
//         input_tx
//             .send(BridgeCommand::ConnectBle(device.clone()))
//             .await
//             .expect("Failed to send command");
//
//         let response = output_rx.recv().await.expect("Failed to receive response");
//
//         // TODO: Fix this test
//         // match response {
//         //     BridgeResponse::ConnectionEstablished(addr) => {
//         //         assert_eq!(addr, device.descriptor.addr);
//         //     }
//         //     _ => panic!("Unexpected response: {:?}", response),
//         // }
//     }
// }
