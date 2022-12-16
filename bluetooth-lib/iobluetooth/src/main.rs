use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, Once},
    thread::{self, sleep},
    time::Duration,
};
extern crate cocoa;
extern crate lazy_static;
extern crate objc;

use cocoa::{
    base::{id, nil},
    foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop},
};
use core_foundation::{
    mach_port::{CFMachPort, CFMachPortCreateRunLoopSource},
    runloop::{
        kCFRunLoopDefaultMode, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun, CFRunLoopStop,
    },
};
use iobluetoothdevice::IOBTDevice;
use libc::uint8_t;
use log::{trace, warn};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel, BOOL},
};
use objc::{sel, sel_impl};
use util::IOBluetoothSDPServiceRecord;

use crate::util::IOBluetoothRFCOMMChannel;

mod inquiry_adapter;
mod inquiry_delegate;
mod iobluetoothdevice;
mod iobluetoothdeviceinquiry;
mod rfcomm_delegate;
mod util;

fn handle_client(mut stream: TcpStream) {
    let channel = open_rfcomm("00001101-0000-1000-8000-00805F9B34FB".to_string(), "AC-12-2F-6A-D2-07".to_string());
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                println!("data: {:?}", &read[0..n]);

                // if channel.is_open() {
                //     let buf = [8, 238, 0, 0, 0, 1, 5, 10, 0, 6];
                //     channel.write_sync(&buf);
                // }

                let data = [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0A, 0x00, 0x06];
                channel.write_sync(&data);
                let buf = match channel.is_open() {
                    true => "OK".as_bytes(),
                    false => "CLOSED".as_bytes(),
                };

                stream.write(&buf).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn on_data_cb(data: &[u8]) {
    println!("data: {:?}", data);
}

fn open_rfcomm(uuid: String, mac_addr: String) -> IOBluetoothRFCOMMChannel{
    let channel_obj;
    unsafe {
        let device = Arc::new(IOBTDevice::new(&mac_addr)); /* "AC-12-2F-6A-D2-07" */
        device.open_connection().unwrap();
        device.perform_sdp_query();
        let record = device.get_service_record(&uuid);
        let mut channel_id: u8 = 0;
        let record = IOBluetoothSDPServiceRecord::new(record);
        record.get_rfcomm_channel_id(&mut channel_id);
        println!("channel: {}", record.get_service_name());
        let delegate = rfcomm_delegate::delegate(on_data_cb);
        channel_obj = device.open_rfcomm_channel_sync(channel_id, delegate);
    }

     IOBluetoothRFCOMMChannel::new_id(channel_obj)
}

fn search() -> String {
    let result = inquiry_adapter::search(Some(Duration::from_secs(3)));
    // for device in result {
    //     if device.name().contains("Liberty") {
    //         return device.name()
    //     }
    // }
    println!("Result: {:?}", &result);
    return "None".to_string();
}

/* This is a cli which runs in the background if --rfcomm is passed and opens a gRPC/socket server to accept write/read commands */
/* We can open channels and search on background thread a long the main RunLoop is running */
/* We must also warn the user that in order to open a rfcomm connection the MUST be disconnected from the device */
fn main() {
    /* We create a process which uses gRPC  */
    // let channel_obj;
    // unsafe {
    //     let device = Arc::new(IOBTDevice::new("AC-12-2F-6A-D2-07"));
    //     device.open_connection();
    //     device.perform_sdp_query();
    //     let record = device.get_service_record("00001101-0000-1000-8000-00805F9B34FB");
    //     let mut channel_id: u8 = 0;
    //     let record = IOBluetoothSDPServiceRecord::new(record);
    //     record.get_rfcomm_channel_id(&mut channel_id);
    //     println!("channel: {}", record.get_service_name());
    //     let delegate = rfcomm_delegate::delegate();
    //     channel_obj = device.open_rfcomm_channel_sync(channel_id, delegate);
    // }
    // let channel = Arc::new(IOBluetoothRFCOMMChannel::new_id(channel_obj));
    // println!("channel: {}", channel.is_open());

    // let clon = channel.clone();

    

    std::thread::spawn(move || {
        // let id = clon.as_id();
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // let c = IOBluetoothRFCOMMChannel::new_id(id);
            handle_client(stream);
        }
    });

    std::thread::spawn(|| {
        // search
        search();
    });

    unsafe {
        CFRunLoopRun();
    }
    // thread::spawn(move || unsafe {
    //     std::thread::sleep(std::time::Duration::from_secs(2));
    //     println!("Starting");
    //     println!("name: {}", chan.get_channel_id());
    //     loop {
    //         if chan.is_open() {
    //             println!("channel is open");
    //         }
    //         let data = [0x01, 0x02, 0x03, 0x04];
    //         chan.write_sync(&data);
    //         std::thread::sleep(std::time::Duration::from_secs(2));
    //     }
    // });
    // println!("chan: {:?}", &channel);
    // let is_open = unsafe {
    //     let channel = &mut *channel as *mut Object;
    //     let is_open: BOOL = msg_send![channel, isOpen];
    //     is_open
    // };
    // println!("is_open: {}", is_open);
    // std::thread::spawn(move || loop {
    //     std::thread::sleep(std::time::Duration::from_secs(2));
    //     println!("is_open: {}", chan.is_open());
    //     let data = [0x01, 0x02, 0x03, 0x04];
    //     chan.write_sync(&data);
    // });
    //unsafe { CFRunLoopRun(); }
    loop {
        println!("done");
        sleep(std::time::Duration::from_secs(5));
    }

    
    // loop {
    //     println!("done after cf");
    //     sleep(std::time::Duration::from_secs(5));
    // }
}
