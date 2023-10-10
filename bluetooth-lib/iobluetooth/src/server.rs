#[cfg(target_os = "macos")]
use core_foundation::runloop::CFRunLoopRun;
#[cfg(target_os = "macos")]
use iobluetoothdevice::IOBTDevice;
#[cfg(target_os = "macos")]
use objc::runtime::Object;
#[cfg(target_os = "macos")]
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "macos")]
use tokio::time::sleep;

#[cfg(target_os = "macos")]
use tonic::{transport::Server, Code, Request, Response, Status};
#[cfg(target_os = "macos")]
use util::{IOBluetoothRFCOMMChannel, IOBluetoothSDPServiceRecord};
#[cfg(target_os = "macos")]
use Searcher::{
    bt_searcher_server::{BtSearcher, BtSearcherServer},
    SearchItem, SearchRequest, SearchResponse,
};
#[cfg(target_os = "macos")]
use RFCOMM::{
    rfcomm_server::{Rfcomm, RfcommServer},
    CloseRfcommChannelRequest, CloseRfcommChannelResponse, OpenRfcommChannelRequest,
    OpenRfcommChannelResponse, RecvRfcommDataRequest, RecvRfcommDataResponse,
    SendRfcommDataRequest, SendRfcommDataResponse,
};
#[cfg(target_os = "macos")]
extern crate lazy_static;
#[cfg(target_os = "macos")]
use lazy_static::lazy_static;

#[cfg(target_os = "macos")]
mod inquiry_adapter;
#[cfg(target_os = "macos")]
mod inquiry_delegate;
#[cfg(target_os = "macos")]
mod iobluetoothdevice;
#[cfg(target_os = "macos")]
mod iobluetoothdeviceinquiry;
#[cfg(target_os = "macos")]
mod rfcomm_delegate;
#[cfg(target_os = "macos")]
mod util;

#[cfg(target_os = "macos")]

pub mod Searcher {
    tonic::include_proto!("Searcher");
}

#[cfg(target_os = "macos")]
pub mod RFCOMM {
    tonic::include_proto!("RFCOMM");
}

#[derive(Debug, Default)]
struct SearchService {}
#[cfg(target_os = "macos")]
#[tonic::async_trait]
impl BtSearcher for SearchService {
    async fn scan(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        let dur = Duration::from_secs(request.into_inner().time_to_scan.unwrap().into());
        let scan_res = inquiry_adapter::search(Some(dur));
        let resp: Vec<SearchItem> = scan_res
            .into_iter()
            .map(|dev| SearchItem {
                name: dev.name().into(),
                addr: dev.address().into(),
                is_connected: dev.is_connected(),
            })
            .collect();
        let reply = SearchResponse { result: resp };
        Ok(Response::new(reply))
    }
}

#[cfg(target_os = "macos")]
#[derive(Default, Debug)]
struct RfcommService {}
#[cfg(target_os = "macos")]

lazy_static! {
    static ref DEVICE: Arc<Mutex<IOBTDevice>> = Arc::new(Mutex::new(IOBTDevice::default()));
    static ref RFCOMM_CHANNEL: Arc<Mutex<IOBluetoothRFCOMMChannel>> =
        Arc::new(Mutex::new(IOBluetoothRFCOMMChannel::default()));
    static ref DATA_STACK: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    static ref LAST_MSG_SEND: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
#[cfg(target_os = "macos")]

fn on_data_cb(data: &[u8]) {
    let mut last_msg_send = LAST_MSG_SEND.lock().unwrap();
    if *last_msg_send {
        let mut stack = DATA_STACK.lock().unwrap();
        stack.push_front(data.to_vec());
        *last_msg_send = false;
    }
}
#[cfg(target_os = "macos")]
#[tonic::async_trait]
impl Rfcomm for RfcommService {
    async fn open_rfcomm_channel(
        &self,
        request: Request<OpenRfcommChannelRequest>,
    ) -> Result<Response<OpenRfcommChannelResponse>, Status> {
        let args = request.into_inner();
        let mut dev = DEVICE
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Device MutexGuard error"))?;
        let mut channel = RFCOMM_CHANNEL
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Channel MutexGuard error"))?;

        // if channel.is_open() {
        //     return Err(Status::already_exists("RFCOMM Channel is already open, close it first."));
        // }
        *dev = IOBTDevice::new(args.addr.as_str());
        dev.open_connection().map_err(|_| {
            Status::new(
                Code::Internal,
                format!("Failed to open connection to {}", args.addr),
            )
        })?;
        dev.perform_sdp_query();
        let svc_record =
            IOBluetoothSDPServiceRecord::new(dev.get_service_record(args.uuid.as_str()));
        let mut channel_id = 0;
        svc_record.get_rfcomm_channel_id(&mut channel_id);
        let delegate = rfcomm_delegate::delegate(on_data_cb);
        let channel_ref = dev.open_rfcomm_channel_sync(channel_id, delegate);
        *channel = IOBluetoothRFCOMMChannel::new_id(channel_ref);
        let reply = OpenRfcommChannelResponse { success: true };
        /* Wait for channel to open, so that if we do a write/read req channel is initialized */
        sleep(Duration::from_millis(1000));
        Ok(Response::new(reply))
    }

    async fn close_rfcomm_channel(
        &self,
        _request: Request<CloseRfcommChannelRequest>,
    ) -> Result<Response<CloseRfcommChannelResponse>, Status> {
        let mut channel = RFCOMM_CHANNEL
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Channel MutexGuard error"))?;
        if channel.is_open() {
            channel.close_channel();
        }
        let reply = CloseRfcommChannelResponse {};
        Ok(Response::new(reply))
    }

    async fn send_rfcomm_data(
        &self,
        request: Request<SendRfcommDataRequest>,
    ) -> Result<Response<SendRfcommDataResponse>, Status> {
        let args = request.into_inner();
        let channel = RFCOMM_CHANNEL
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Channel MutexGuard error"))?;
        let mut last_msg_send = LAST_MSG_SEND
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Last msg send MutexGuard error"))?;
        if channel.is_open() {
            channel.write_sync(&args.data);
            *last_msg_send = true;
            let reply = SendRfcommDataResponse { success: true };
            /* Wait for data to actually be sent */
            sleep(Duration::from_millis(200));
            Ok(Response::new(reply))
        } else {
            Err(Status::new(Code::Cancelled, "Channel is not open"))
        }
    }

    async fn recv_rfcomm_data(
        &self,
        _request: Request<RecvRfcommDataRequest>,
    ) -> Result<Response<RecvRfcommDataResponse>, Status> {
        sleep(Duration::from_millis(200));
        let data_to_send = DATA_STACK
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Data stack MutexGuard error"))?
            .pop_front();
        if let Some(data) = data_to_send {
            let reply = RecvRfcommDataResponse { data };
            Ok(Response::new(reply))
        } else {
            Err(Status::new(Code::Cancelled, "No data to send"))
        }
    }
}
#[cfg(target_os = "macos")]
/* We should use 1 thread for tokio */
#[tokio::main(flavor = "current_thread")]
async fn launch_rpc_server() {
    let address = "[::1]:55777".parse().unwrap();
    let search_svc = SearchService::default();
    let rfcomm_svc = RfcommService::default();

    Server::builder()
        .add_service(BtSearcherServer::new(search_svc))
        .add_service(RfcommServer::new(rfcomm_svc))
        .serve(address)
        .await
        .unwrap();
}
#[cfg(target_os = "macos")]

fn main() {
    std::thread::spawn(|| launch_rpc_server());
    unsafe {
        CFRunLoopRun();
    }
}
#[cfg(not(target_os = "macos"))]
fn main() {}
