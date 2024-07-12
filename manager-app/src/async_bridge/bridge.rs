use std::sync::Arc;

use log::{debug, info, trace};

use manager_fut::TokioFuture;
use tokio::sync::{mpsc, Mutex};

use super::{
    AddrWrappedPayload, BridgeCommand, BridgeResponse, ConnectionFailedResponse,
    SetEqualizerPayload, TaggedStateResponse,
};
use soundcore_lib::models::MonoEQ;
use soundcore_lib::{
    ble::BLEConnectionManager,
    device::SoundcoreBLEDevice,
    device_manager::{create_device_manager, DeviceManager},
    models::EQConfiguration,
};

struct CommandLoopState<B: BLEConnectionManager> {
    manager: DeviceManager<B, TokioFuture>,
}

impl<B: BLEConnectionManager> CommandLoopState<B> {
    fn new(manager: DeviceManager<B, TokioFuture>) -> Self {
        Self { manager }
    }
}

pub async fn async_bridge(
    mut input_rx: mpsc::Receiver<BridgeCommand>,
    output_tx: mpsc::Sender<BridgeResponse>,
) {
    tokio::spawn(async move {
        let manager = create_device_manager().await;

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
        BridgeCommand::DisconnectAll => command_loop_state
            .lock()
            .await
            .manager
            .disconnect_all()
            .await
            .map(|_| BridgeResponse::DisconnectedAll),
        BridgeCommand::Connect(d) => {
            let addr = d.clone().descriptor.addr;
            let device = command_loop_state.lock().await.manager.connect(d).await;
            let addr_clone = addr.clone();
            if let Ok(device) = device {
                // Get the state channel and listen for changes in the background
                let mut state_channel = device.state_channel().await;

                // TODO: Investigate this
                #[allow(clippy::let_underscore_future)]
                let _ = tokio::task::spawn(async move {
                    info!("Listening for state changes for {:?}", addr_clone);
                    while let Ok(()) = state_channel.changed().await {
                        let state = state_channel.borrow().clone();
                        trace!(
                            "Got new state {:?} for {:?}, sending event...",
                            state,
                            addr_clone
                        );
                        let res = output_tx
                            .send(BridgeResponse::NewState(TaggedStateResponse {
                                addr: addr_clone.clone(),
                                state,
                            }))
                            .await;

                        if let Err(e) = res {
                            debug!("Failed to send new state event: {:?}", e);
                        }
                    }
                    // TODO: Send a StateChannelClosed event
                    trace!("State channel for {:?} closed", addr_clone);
                });
                Ok(BridgeResponse::ConnectionEstablished(TaggedStateResponse {
                    addr,
                    state: device.latest_state().await,
                }))
            } else {
                Ok(BridgeResponse::ConnectionFailed(ConnectionFailedResponse {
                    addr,
                    reason: device.err().unwrap().to_string(),
                }))
            }
        }
        BridgeCommand::SetSoundMode(payload) => {
            let addr_clone = payload.addr.clone();
            let device = command_loop_state
                .lock()
                .await
                .manager
                .get_device(payload.addr)
                .await;

            if let Some(device) = device {
                trace!("Setting sound mode for {:?}", addr_clone);
                let res = device.set_sound_mode(payload.payload).await;
                if res.is_ok() {
                    Ok(BridgeResponse::SoundModeUpdated(addr_clone))
                } else {
                    Ok(BridgeResponse::GenericError(res.err().unwrap().to_string()))
                }
            } else {
                Ok(BridgeResponse::DeviceNotFound(addr_clone))
            }
        }
        BridgeCommand::SetEqualizer(payload) => {
            let device = command_loop_state
                .lock()
                .await
                .manager
                .get_device(payload.addr.clone())
                .await;

            if let Some(device) = device {
                trace!("Setting equalizer for {:?}", payload.addr);
                return handle_set_eq::<B>(device, payload).await;
            } else {
                Ok(BridgeResponse::DeviceNotFound(payload.addr))
            }
        }
    }
    .map_err(|e| BridgeResponse::GenericError(e.to_string()))
    .unwrap_or_else(|e| e)
}

async fn handle_set_eq<B: BLEConnectionManager>(
    device: Arc<SoundcoreBLEDevice<<B as BLEConnectionManager>::Connection, TokioFuture>>,
    wrapped_payload: AddrWrappedPayload<SetEqualizerPayload>,
) -> BridgeResponse {
    let eq_configuration = match wrapped_payload.payload {
        SetEqualizerPayload::SetCustomEqualizer(values) => {
            EQConfiguration::mono_custom(MonoEQ::from_signed_bytes(values))
        }
        SetEqualizerPayload::SetEqualizerPreset(profile) => {
            EQConfiguration::stereo_with_profile(profile)
        }
    };

    match device.set_eq(eq_configuration).await {
        Ok(_) => BridgeResponse::EqualizerUpdated(wrapped_payload.addr),
        Err(e) => BridgeResponse::GenericError(e.to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn create_bridge() -> (mpsc::Sender<BridgeCommand>, mpsc::Receiver<BridgeResponse>) {
        let (input_tx, input_rx) = mpsc::channel(1);
        let (output_tx, output_rx) = mpsc::channel(1);
        async_bridge(input_rx, output_tx).await;
        (input_tx, output_rx)
    }

    #[tokio::test]
    async fn should_handle_scan_command_and_produce_response() {
        let (input_tx, mut output_rx) = create_bridge().await;
        input_tx
            .send(BridgeCommand::Scan)
            .await
            .expect("Failed to send command");

        let response = output_rx.recv().await.expect("Failed to receive response");

        match response {
            BridgeResponse::ScanResult(res) => {
                assert!(!res.is_empty());
            }
            _ => panic!("Unexpected response: {:?}", response),
        }
    }

    #[tokio::test]
    async fn should_handle_connect_command_and_produce_response() {
        let (input_tx, mut output_rx) = create_bridge().await;
        input_tx
            .send(BridgeCommand::Scan)
            .await
            .expect("Failed to send command");

        let scan_response = output_rx.recv().await.expect("Failed to receive response");

        let devices = match scan_response {
            BridgeResponse::ScanResult(res) => res,
            _ => panic!("Unexpected response: {:?}", scan_response),
        };

        let device = devices.first().unwrap();

        input_tx
            .send(BridgeCommand::Connect(device.clone()))
            .await
            .expect("Failed to send command");

        let response = output_rx.recv().await.expect("Failed to receive response");

        // TODO: Fix this test
        match response {
            BridgeResponse::ConnectionEstablished(resp) => {
                assert_eq!(resp.addr, device.descriptor.addr);
            }
            _ => panic!("Unexpected response: {:?}", response),
        }
    }
}
