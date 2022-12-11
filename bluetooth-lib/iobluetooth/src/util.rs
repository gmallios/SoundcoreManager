use std::{ffi::CStr, fmt::Formatter};

use cocoa::{
    base::{id, nil, BOOL},
    foundation::{NSData, NSString},
};
use libc::{c_char, c_void};
use objc::{class, msg_send, rc::StrongPtr, runtime::{Class, Object}, sel, sel_impl};
use std::fmt::Debug;

pub type IOReturn = libc::c_int;

pub fn string_to_string(nsstring: id) -> String {
    if nsstring == nil {
        return String::from("nil");
    }
    unsafe {
        String::from(
            CStr::from_ptr(string_utf8string(nsstring))
                .to_str()
                .unwrap(),
        )
    }
}

pub fn string_utf8string(nsstring: id) -> *const c_char {
    unsafe {
        let utf8string: *const c_char = msg_send![nsstring, UTF8String];
        utf8string
    }
}

/* Initialize CBUUID from String then convert it to IOBluetoothSDPUUID */
pub fn sdpuuid_from_str(uuid: &str) -> id {
    unsafe {
        let nsstr = NSString::alloc(nil).init_str(uuid);
        let id: id = msg_send![class!(CBUUID), UUIDWithString: nsstr];
        let uuid_data: id = msg_send![id, data];
        let uuid: id = msg_send![
            Class::get("IOBluetoothSDPUUID").unwrap(),
            uuidWithData: uuid_data
        ];
        uuid
    }
}

pub struct IOBluetoothSDPServiceRecord {
    record: id,
}

unsafe impl Send for IOBluetoothSDPServiceRecord {}
unsafe impl Sync for IOBluetoothSDPServiceRecord {}

impl IOBluetoothSDPServiceRecord {
    pub fn new(record: id) -> IOBluetoothSDPServiceRecord {
        IOBluetoothSDPServiceRecord { record }
    }

    pub fn as_id(&self) -> id {
        self.record
    }

    pub fn get_rfcomm_channel_id(&self, write_ref: &mut u8) {
        unsafe {
            let channel_id: u8 = msg_send![self.record, getRFCOMMChannelID: write_ref];
        }
    }

    pub fn get_l2cap_psm(&self, write_ref: &mut u16) {
        unsafe {
            let psm: u16 = msg_send![self.record, getL2CAPPSM: write_ref];
        }
    }

    pub fn get_service_name(&self) -> String {
        unsafe {
            let name: id = msg_send![self.record, getServiceName];
            string_to_string(name)
        }
    }
}

pub struct IOBluetoothRFCOMMChannel {
    channel: StrongPtr,
}

unsafe impl Send for IOBluetoothRFCOMMChannel {}
unsafe impl Sync for IOBluetoothRFCOMMChannel {}

impl Debug for IOBluetoothRFCOMMChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOBluetoothRFCOMMChannel")
            .field("channel_id", &self.get_channel_id())
            .field("is_open", &self.is_open())
            .finish()
    }
}

/* Default impl is null */
impl Default for IOBluetoothRFCOMMChannel {
    fn default() -> Self {
        unsafe {
            let null_ptr = std::ptr::null::<Object>();
            Self::new_id(null_ptr as id)
        }
    }
}

impl IOBluetoothRFCOMMChannel {
    pub fn new() -> IOBluetoothRFCOMMChannel {
        unsafe {
            let chan: id = { msg_send![class!(IOBluetoothRFCOMMChannel), alloc] };
            let channel: id = { msg_send![chan, init] };

            IOBluetoothRFCOMMChannel {
                channel: StrongPtr::new(channel),
            }
        }
    }

    pub fn new_id(channel: id) -> IOBluetoothRFCOMMChannel {
        let channel = unsafe { StrongPtr::new(channel) };
        IOBluetoothRFCOMMChannel { channel }
    }

    pub fn as_id(&self) -> id {
        *self.channel
    }

    pub fn close_channel(&self) {
        unsafe {
            let _ret: IOReturn = msg_send![*self.channel, closeChannel];
        }
    }

    pub fn get_device(&self) -> id /* IOBluetoothDevice */ {
        unsafe {
            let device: id = msg_send![*self.channel, getDevice];
            device
        }
    }

    pub fn is_open(&self) -> bool {
        unsafe {
            let is_open: BOOL = msg_send![*self.channel, isOpen];
            is_open
        }
    }

    pub fn write_sync(&self, data: &[u8]) {
        unsafe {
            let len = data.len();
            let data_ptr: *const *mut c_void = data.as_ptr() as *const *mut c_void;
            let _ret: IOReturn = msg_send![*self.channel, writeSync: data_ptr length: len];
        }
    }

    pub fn get_channel_id(&self) -> u8 {
        unsafe {
            let channel_id: u8 = msg_send![*self.channel, getChannelID];
            channel_id
        }
    }
}
