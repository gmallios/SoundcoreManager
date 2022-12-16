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

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

struct BlockingClient {
    client: RfcommClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl BlockingClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        println!("before rt");
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        println!("after rt");
        let client = rt.block_on(RfcommClient::connect(dst))?;
        println!("after client");
        Ok(Self { client, rt })
    }

    pub fn send_rfcomm(
        &mut self,
        request: impl tonic::IntoRequest<SendRfcommDataRequest>,
    ) -> Result<tonic::Response<SendRfcommDataResponse>, tonic::Status> {
        println!("sending");
        let res = self.rt.block_on(self.client.send_rfcomm_data(request));
        println!("sent");
        res 
    }
}

/* TODO: Create helper for starting daemon(server) */
/* TODO: Use scan() as base for tokio runtime detection and handling */

pub fn send_rfcomm(data: Vec<u8>, h: &Handle) {
    // let (handle, rt) = get_runtime_handle();

    let rf_send_req = tonic::Request::new(rfcomm::SendRfcommDataRequest { data: data });
    let mut rf_client = h.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    let _rf_send_resp = h.block_on(rf_client.send_rfcomm_data(rf_send_req)).unwrap();

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
}

pub fn recv_rfcomm(h: Handle) -> Vec<u8> {
    let rf_recv_req = tonic::Request::new(rfcomm::RecvRfcommDataRequest {});
    let mut rf_client = h.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    let rf_recv_resp = h.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
    // match rt {
    //     Some(_) => {
    //         let mut rf_client =
    //             handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    //         let rf_recv_resp = handle.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
    //         rf_recv_resp.into_inner().data
    //     }
    //     None => {
    //         let mut rf_client =
    //             futures::executor::block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    //         let rf_recv_resp =
    //             futures::executor::block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
    //         rf_recv_resp.into_inner().data
    //     }
    // }
    // let mut rf_client = handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
    // let rf_recv_resp = handle.block_on(rf_client.recv_rfcomm_data(rf_recv_req)).unwrap();
    rf_recv_resp.into_inner().data
}

pub fn open_rfcomm(uuid: String, addr: String) {
    let (handle, rt) = get_runtime_handle();
    let rf_req = tonic::Request::new(rfcomm::OpenRfcommChannelRequest {
        addr: addr,
        uuid: uuid,
    });
    match rt {
        Some(_) => {
            let mut rf_client =
                handle.block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
            let _rf_resp = handle.block_on(rf_client.open_rfcomm_channel(rf_req)).unwrap();
        }
        None => {
            let mut rf_client =
                futures::executor::block_on(RfcommClient::connect("http://[::1]:8080")).unwrap();
            let _rf_resp = futures::executor::block_on(rf_client.open_rfcomm_channel(rf_req)).unwrap();
        }
    }
}

pub fn scan() -> Vec<SearchItem> {
    let (handle, rt) = get_runtime_handle();
    let request = tonic::Request::new(SearchRequest {
        time_to_scan: Some(5),
    });
    let scan_res: Vec<SearchItem> = match rt {
        Some(_) => {
            let mut client =
                handle.block_on(BtSearcherClient::connect("http://[::1]:8080"))
                    .unwrap();
            let resp = handle.block_on(client.scan(request)).unwrap();
            resp.into_inner().result
        }
        None => {
            let mut client =
                futures::executor::block_on(BtSearcherClient::connect("http://[::1]:8080"))
                    .unwrap();     
            let resp = futures::executor::block_on(client.scan(request)).unwrap();
            resp.into_inner().result           
        } 
    };
    std::thread::sleep(std::time::Duration::from_millis(500));
    scan_res
}

fn get_runtime_handle() -> (Handle, Option<Runtime>) {
    match Handle::try_current() {
        Ok(h) => (h, None),
        Err(_) => {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            (rt.handle().clone(), Some(rt))
        }
    }
}
