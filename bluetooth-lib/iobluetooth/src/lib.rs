use rfcomm::{
    rfcomm_client::RfcommClient, OpenRfcommChannelRequest, OpenRfcommChannelResponse,
    RecvRfcommDataRequest, RecvRfcommDataResponse, SendRfcommDataRequest, SendRfcommDataResponse,
};
use searcher::{bt_searcher_client::BtSearcherClient, SearchItem, SearchRequest};
use tokio::runtime::{Handle, Runtime, Builder};

pub mod searcher {
    tonic::include_proto!("Searcher");
}

pub mod rfcomm {
    tonic::include_proto!("RFCOMM");
}

/* TODO: Create helper for starting daemon(server) */
/* TODO: Use scan() as base for tokio runtime detection and handling */

// pub fn send_rfcomm(data: Vec<u8>, h: &Handle) {
//     // let (handle, rt) = get_runtime_handle();

//     let rf_send_req = tonic::Request::new(rfcomm::SendRfcommDataRequest { data: data });
//     let mut rf_client = h.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//     let _rf_send_resp = h.block_on(rf_client.send_rfcomm_data(rf_send_req)).unwrap();

    // match rt {
    //     Some(_) => {
    //         let mut rf_client =
    //             handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    //         let _rf_send_resp = handle.block_on(rf_client.send_rfcomm_data(rf_send_req)).unwrap();
    //     }
    //     None => {
    //         let mut rf_client =
    //             futures::executor::block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    //         let _rf_send_resp = futures::executor::block_on(rf_client.send_rfcomm_data(rf_send_req))
    //             .unwrap();
    //     }
    // }
    // let mut rf_client = handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    // let _rf_send_resp = handle.block_on(rf_client.send_rfcomm_data(rf_send_req)).unwrap();
// }

// pub fn recv_rfcomm(h: Handle) -> Vec<u8> {
//     let rf_recv_req = tonic::Request::new(rfcomm::RecvRfcommDataRequest {});
//     let mut rf_client = h.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//     let rf_recv_resp = h.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
//     // match rt {
//     //     Some(_) => {
//     //         let mut rf_client =
//     //             handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//     //         let rf_recv_resp = handle.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
//     //         rf_recv_resp.into_inner().data
//     //     }
//     //     None => {
//     //         let mut rf_client =
//     //             futures::executor::block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//     //         let rf_recv_resp =
//     //             futures::executor::block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
//     //         rf_recv_resp.into_inner().data
//     //     }
//     // }
//     // let mut rf_client = handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//     // let rf_recv_resp = handle.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
//     rf_recv_resp.into_inner().data
// }

// pub fn open_rfcomm(uuid: String, addr: String) {
//     let (handle, rt) = get_runtime_handle();
//     let rf_req = tonic::Request::new(rfcomm::OpenRfcommChannelRequest {
//         addr: addr,
//         uuid: uuid,
//     });
//     match rt {
//         Some(_) => {
//             let mut rf_client =
//                 handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//             let _rf_resp = handle.block_on(rf_client.open_rfcomm_channel(rf_req)).unwrap();
//         }
//         None => {
//             let mut rf_client =
//                 futures::executor::block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
//             let _rf_resp = futures::executor::block_on(rf_client.open_rfcomm_channel(rf_req)).unwrap();
//         }
//     }
// }

pub async fn scan() -> Vec<SearchItem> {
    let request = tonic::Request::new(SearchRequest {
        time_to_scan: Some(5),
    });
    let mut client = BtSearcherClient::connect("http://[::1]:8080").await.unwrap();
    let resp = client.scan(request).await.unwrap();
    resp.into_inner().result
}
