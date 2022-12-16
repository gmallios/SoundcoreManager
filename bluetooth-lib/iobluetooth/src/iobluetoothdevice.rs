use std::fmt::Debug;

use cocoa::base::{id, BOOL};
use libc::{uint32_t, uint8_t, KERN_SUCCESS};
use objc::{
    class, msg_send,
    rc::StrongPtr,
    runtime::{Class, Object},
    sel, sel_impl,
};

use crate::util::{string_to_string, IOBluetoothRFCOMMChannel, IOReturn};

#[derive(Clone, Copy)]
pub struct IOBTDevice {
    device: id, /* IOBluetoothDevice */
}

impl Debug for IOBTDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOBTDevice")
            .field("name", &self.name())
            .field("mac", &self.address())
            .finish()
    }
}

impl Default for IOBTDevice {
    fn default() -> Self {
        unsafe {
            let null_ptr = std::ptr::null::<Object>();
            Self::from_id(null_ptr as id)
        }
    }
}

unsafe impl Send for IOBTDevice {}
unsafe impl Sync for IOBTDevice {}

impl IOBTDevice {
    /* Form of NSString is xx:xx:xx:xx:xx:xx */
    pub fn new(add: &str) -> IOBTDevice {
        /* Convert string to NSString */
        unsafe {
            let addr = cocoa::foundation::NSString::init_str(
                cocoa::foundation::NSString::alloc(cocoa::base::nil),
                add,
            );

            IOBTDevice {
                device: msg_send![
                    Class::get("IOBluetoothDevice").unwrap(),
                    deviceWithAddressString: addr /* NSString */
                ],
            }
        }
    }

    pub fn from_id(device: id /* IOBluetoothDevice object */) -> IOBTDevice {
        IOBTDevice { device }
    }

    pub fn as_id(&self) -> id {
        self.device
    }

    pub fn address(&self) -> String {
        unsafe {
            let addr: id = msg_send![self.device, addressString];
            string_to_string(addr)
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let name: id = msg_send![self.device, name];
            string_to_string(name)
        }
    }

    pub fn is_paired(&self) -> bool {
        unsafe {
            let is_paired: BOOL = msg_send![self.device, isPaired];
            is_paired
        }
    }

    pub fn is_connected(&self) -> bool {
        unsafe {
            let is_connected: BOOL = msg_send![self.device, isConnected];
            is_connected
        }
    }

    pub fn open_connection(&self) -> Result<(), ()> {
        unsafe {
            let ret: IOReturn = msg_send![self.device, openConnection];
            match ret {
                KERN_SUCCESS => Ok(()),
                _ => Err(()),
            } 
        }
    }

    pub fn close_connection(&self) {
        unsafe {
            let _ret: IOReturn = msg_send![self.device, closeConnection];
        }
    }

    pub fn perform_sdp_query(&self) {
        unsafe {
            let _ret: IOReturn = msg_send![self.device, performSDPQuery:0 ];
        }
    }

    // pub fn perform_sdp_query_uuid(&self, uuid: Vec<String>){
    //     /* Convert each String to IOBluetoothSDPUUID then push them to a NSArray  */
    // }

    pub fn open_rfcomm_channel_sync(&self, channel_id: u8, delegate: id) -> id /* IOBluetoothRFCOMMChannel */
    {
        unsafe {
            let val: *mut Object = msg_send![class!(IOBluetoothRFCOMMChannel), alloc];
            let addr: *const *mut Object = &val;
            // println!("ret_before: {:?}", add);
            //println!("val_before: {:?}", val);
            let _ret: IOReturn = msg_send![self.device, openRFCOMMChannelSync:addr /* Docs call for double pointer */
                withChannelID:channel_id delegate:delegate];
            // println!("ret_after: {:?}", add);
            //println!("val_after: {:?}", val);
            val
        }
    }

    pub fn get_service_record(&self, uuid: &str) -> id {
        unsafe {
            let uuid = crate::util::sdpuuid_from_str(uuid);
            let service_record: id = msg_send![self.device, getServiceRecordForUUID: uuid];
            service_record
        }
    }
}
