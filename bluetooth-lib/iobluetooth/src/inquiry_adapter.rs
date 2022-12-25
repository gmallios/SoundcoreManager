use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use itertools::Itertools;

use cocoa::{
    base::id,
    foundation::{NSDate, NSRunLoop},
};
use core_foundation::{
    base::{kCFAllocatorDefault, kCFAllocatorNull, Boolean},
    mach_port::CFIndex,
    runloop::{CFRunLoopGetCurrent, CFRunLoopRun, CFRunLoopRunInMode, CFRunLoopStop},
    string::{kCFStringEncodingUTF8, CFString, CFStringCreateWithBytesNoCopy, CFStringRef},
};
use lazy_static::lazy_static;
use objc::{
    msg_send,
    rc::StrongPtr,
    runtime::{Class, Object},
    sel, sel_impl, class,
};

use crate::{
    inquiry_delegate::InquiryDelegate::{self},
    iobluetoothdevice::IOBTDevice,
    iobluetoothdeviceinquiry::IOBluetoothDeviceInquiry,
};

pub struct InquiryAdapter {
    inquiry: StrongPtr,
    delegate: StrongPtr, /* We need to hold the delegate otherwise it gets dropped */
}
unsafe impl Send for InquiryAdapter {}
unsafe impl Sync for InquiryAdapter {}

lazy_static! {
    static ref SCAN_RESULTS: Arc<Mutex<Vec<IOBTDevice>>> = Arc::new(Mutex::new(Vec::new()));
}

impl InquiryAdapter {
    pub fn new() -> Self {
        unsafe {
            let d = InquiryDelegate::delegate(Self::add_device_cb as fn(IOBTDevice));
            let delegate = StrongPtr::new(d);
            let inquiry = IOBluetoothDeviceInquiry::new(*delegate);
            Self {
                inquiry: StrongPtr::new(inquiry.as_obj()),
                delegate: delegate,
            }
        }
    }
    pub fn scan(&self) {
        unsafe {
            let () = msg_send![*self.inquiry, start];
        }
    }

    pub fn stop(&self) {
        unsafe {
            let () = msg_send![*self.inquiry, stop];
            // CFRunLoopStop(CFRunLoopGetCurrent());
        }
    }

    fn add_device_cb(device: IOBTDevice) {
        SCAN_RESULTS.lock().unwrap().push(device);
    }
}

pub fn search(duration: Option<Duration>) -> Vec<IOBTDevice> {
    SCAN_RESULTS.lock().unwrap().clear();
    let adapt = Arc::new(InquiryAdapter::new());
    if let Some(dur) = duration {
        let a = adapt.clone();
        std::thread::spawn(move || {
            std::thread::sleep(dur);
            a.stop();
        });
    }
    adapt.scan();
    if let Some(dur) = duration {
        std::thread::sleep(dur);
    }
    unsafe {
        let paired: id = msg_send![class!(IOBluetoothDevice), pairedDevices];
        // paired is an NSArray of IOBluetoothDevice
        let count: CFIndex = msg_send![paired, count];
        for i in 0..count {
            let dev: id = msg_send![paired, objectAtIndex: i];
            SCAN_RESULTS.lock().unwrap().push(IOBTDevice::from_id(dev));
        }
    }
    /* Collect results */
    SCAN_RESULTS.lock().unwrap().clone().into_iter().unique_by(|d| d.address()).collect()
}
