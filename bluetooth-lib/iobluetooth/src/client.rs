use rfcomm::{
    rfcomm_client::RfcommClient, OpenRfcommChannelRequest, OpenRfcommChannelResponse,
    RecvRfcommDataRequest, RecvRfcommDataResponse, SendRfcommDataRequest, SendRfcommDataResponse,
};
use searcher::{bt_searcher_client::BtSearcherClient, SearchItem, SearchRequest};
use tokio::runtime::{Handle, Runtime};

pub mod searcher {
    tonic::include_proto!("Searcher");
}

pub mod rfcomm {
    tonic::include_proto!("RFCOMM");
}
fn main() {
    // let scan_res = scan();
    // println!("scan_res: {:?}", scan_res);
    // open_rfcomm(
    //     "00001101-0000-1000-8000-00805F9B34FB".to_string(),
    //     "AC-12-2F-6A-D2-07".to_string(),
    // );
    // let rf_req = tonic::Request::new(rfcomm::OpenRfcommChannelRequest {
    //     addr: "AC-12-2F-6A-D2-07".to_string(),
    //     uuid: "00001101-0000-1000-8000-00805F9B34FB".to_string(),
    // });
    // let mut rf_client = RfcommClient::connect("http://[::1]:8080").await.unwrap();
    // // let rf_resp = rf_client.open_rfcomm_channel(rf_req).await.unwrap();
    // // println!("got rf_resp {:?}", rf_resp);
    //open_rfcomm("00001101-0000-1000-8000-00805F9B34FB".to_string(), "AC-12-2F-6A-D2-07".to_string());
    let data = [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0A, 0x00, 0x06];
    send_rfcomm_data(data.to_vec());
    let recv = recv_rfcomm_data();
    println!("recv: {:?}", recv);
    let recv = recv_rfcomm_data();
    println!("recv: {:?}", recv);
    // let rf_send_req = tonic::Request::new(rfcomm::SendRfcommDataRequest {
    //     data: data.to_vec(),
    // });
    // let rf_send_resp = rf_client.send_rfcomm_data(rf_send_req).await.unwrap();
    // println!("got rf_send_resp {:?}", rf_send_resp);
}

#[tokio::main]
async fn send_rfcomm_data(data: Vec<u8>) {
    let rf_send_req = tonic::Request::new(rfcomm::SendRfcommDataRequest { data: data });
    let mut rf_client = RfcommClient::connect("http://[::1]:8080").await.unwrap();
    let _rf_send_resp = rf_client.send_rfcomm_data(rf_send_req).await.unwrap();
}

#[tokio::main]
async fn recv_rfcomm_data() -> Vec<u8> {
    let rf_recv_req = tonic::Request::new(rfcomm::RecvRfcommDataRequest {});
    let mut rf_client = RfcommClient::connect("http://[::1]:8080").await.unwrap();
    let rf_recv_resp = rf_client.recv_rfcomm_data(rf_recv_req).await.unwrap();
    rf_recv_resp.into_inner().data
}


#[tokio::main]
async fn open_rfcomm(uuid: String, addr: String) {
    let rf_req = tonic::Request::new(rfcomm::OpenRfcommChannelRequest {
        addr: addr,
        uuid: uuid,
    });
    let mut rf_client = RfcommClient::connect("http://[::1]:8080").await.unwrap();
    let _rf_resp = rf_client.open_rfcomm_channel(rf_req).await.unwrap();
}

#[tokio::main]
async fn scan() -> Vec<SearchItem> {
    let mut client =
        futures::executor::block_on(BtSearcherClient::connect("http://[::1]:8080")).unwrap();
    let request = tonic::Request::new(SearchRequest {
        time_to_scan: Some(5),
    });
    let resp = futures::executor::block_on(client.scan(request)).unwrap();
    let scan_res: Vec<SearchItem> = resp.into_inner().result;
    scan_res
}
