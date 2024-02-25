use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{mpsc, Mutex};

use soundcore_lib::device::SoundcoreBLEDevice;
use soundcore_lib::{
    ble::BLEConnectionManager,
    device_manager::{create_device_manager, DeviceManager},
};

use super::{BridgeCommand, BridgeResponse};

struct CommandLoopState<B: BLEConnectionManager> {
    manager: DeviceManager<B>,
    app_handle: AppHandle,
    devices: Vec<Arc<SoundcoreBLEDevice<B::Connection>>>,
}

impl<B: BLEConnectionManager> CommandLoopState<B> {
    fn new(manager: DeviceManager<B>, app_handle: AppHandle) -> Self {
        Self {
            manager,
            app_handle,
            devices: Vec::new(),
        }
    }
}

pub async fn async_bridge(
    mut input_rx: mpsc::Receiver<BridgeCommand>,
    output_tx: mpsc::Sender<BridgeResponse>,
    app_handle: AppHandle,
) {
    let command_loop = tokio::spawn(async move {
        let manager = create_device_manager().await;
        let command_loop_state = Arc::new(Mutex::new(CommandLoopState::new(manager, app_handle)));
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
        BridgeCommand::ScanBle => command_loop_state
            .lock()
            .await
            .manager
            .ble_scan(None)
            .await
            .map(BridgeResponse::ScanResult),
        BridgeCommand::DisconnectBle(addr) => {
            let addr_clone = addr.clone();
            command_loop_state
                .lock()
                .await
                .manager
                .disconnect(addr)
                .await
                .map(|_| BridgeResponse::Disconnected(addr_clone))
        }
        BridgeCommand::ConnectBle(d) => {
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
                        output_tx
                            .send(BridgeResponse::NewState((addr_clone.clone(), state)))
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
