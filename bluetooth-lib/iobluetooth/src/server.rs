use core_foundation::runloop::CFRunLoopRun;
use iobluetoothdevice::IOBTDevice;
use objc::runtime::Object;
use std::{
    sync::{Arc, Mutex},
    time::Duration, collections::VecDeque,
};
use tonic::{transport::Server, Code, Request, Response, Status};
use Searcher::{
    bt_searcher_server::{BtSearcher, BtSearcherServer},
    SearchItem, SearchRequest, SearchResponse,
};

use util::{IOBluetoothRFCOMMChannel, IOBluetoothSDPServiceRecord};
use RFCOMM::{
    rfcomm_server::{Rfcomm, RfcommServer},
    OpenRfcommChannelRequest, OpenRfcommChannelResponse,
    CloseRfcommChannelRequest, CloseRfcommChannelResponse, 
    RecvRfcommDataRequest, RecvRfcommDataResponse, 
    SendRfcommDataRequest, SendRfcommDataResponse,
};

extern crate lazy_static;
use lazy_static::lazy_static;

mod inquiry_adapter;
mod inquiry_delegate;
mod iobluetoothdevice;
mod iobluetoothdeviceinquiry;
mod rfcomm_delegate;
mod util;

pub mod Searcher {
    tonic::include_proto!("Searcher");
}

pub mod RFCOMM {
    tonic::include_proto!("RFCOMM");
}

#[derive(Debug, Default)]
struct SearchService {}

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
#[derive(Default, Debug)]
struct RfcommService {}

lazy_static! {
    static ref DEVICE: Arc<Mutex<IOBTDevice>> = Arc::new(Mutex::new(IOBTDevice::default()));
    static ref RFCOMM_CHANNEL: Arc<Mutex<IOBluetoothRFCOMMChannel>> =
        Arc::new(Mutex::new(IOBluetoothRFCOMMChannel::default()));
    static ref DATA_STACK: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    static ref LAST_MSG_SEND: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); 
}

fn on_data_cb(data: &[u8]) {
    let mut last_msg_send = LAST_MSG_SEND
        .lock()
        .unwrap();
    if *last_msg_send {
        let mut stack = DATA_STACK
            .lock()
            .unwrap();
        stack.push_front(data.to_vec());
        *last_msg_send = false;
    }
}

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
        // if dev.is_connected() {
        //     dev.close_connection();
        //     std::thread::sleep(Duration::from_millis(500));
        // }
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
        std::thread::sleep(Duration::from_millis(1000));
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
            std::thread::sleep(Duration::from_millis(200));
            Ok(Response::new(reply))
        } else {
            Err(Status::new(Code::Cancelled, "Channel is not open"))
        }
    }

    async fn recv_rfcomm_data(
        &self,
        _request: Request<RecvRfcommDataRequest>,
    ) -> Result<Response<RecvRfcommDataResponse>, Status> {
        std::thread::sleep(Duration::from_millis(200));
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

/* We should use 1 thread for tokio */
#[tokio::main]
async fn launch_rpc_server() {
    let address = "[::1]:8080".parse().unwrap();
    let search_svc = SearchService::default();
    let rfcomm_svc = RfcommService::default();

    Server::builder()
        .add_service(BtSearcherServer::new(search_svc))
        .add_service(RfcommServer::new(rfcomm_svc))
        .serve(address)
        .await
        .unwrap();
}

fn main() {
    std::thread::spawn(|| launch_rpc_server());
    unsafe {
        CFRunLoopRun();
    }
}
