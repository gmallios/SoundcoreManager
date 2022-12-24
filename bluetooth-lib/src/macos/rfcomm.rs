use std::{sync::Arc, cell::{Ref, RefCell}};
use async_trait::async_trait;
use tokio::runtime::{Runtime, Handle};

use crate::{RFCOMMClient, BthError};

pub struct RFCOMM {
}

#[async_trait]
impl RFCOMMClient for RFCOMM {
    fn new() -> Result<Self, BthError> {
        Ok(RFCOMM {})
    }

    async fn connect_uuid(&mut self, bt_addr: crate::BluetoothAdrr, uuid: &str) -> Result<(), crate::BthError> {
        Ok(())
    }

    async fn connect_port(&mut self, address: crate::BluetoothAdrr, port: u32) -> Result<(), crate::BthError> {
        todo!()
    }

    async fn send(&self, data: &[u8]) -> Result<(), crate::BthError> {
        Ok(())
    }

    async fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, crate::BthError> {
        let res = Vec::with_capacity(num_of_bytes);
        Ok(res)
    }

    fn close(&self) {
        todo!()
    }
}