#![cfg(target_os = "macos")]
pub mod searcher {
    tonic::include_proto!("Searcher");
}

pub mod rfcomm {
    tonic::include_proto!("RFCOMM");
}

pub use tonic::transport::Channel as TonicTransportChannel;
pub use tonic::Request;
