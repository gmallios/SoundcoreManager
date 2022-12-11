use cocoa::base::id;
use objc::{msg_send, sel, sel_impl, runtime::Class};

pub struct IOBluetoothDeviceInquiry {
    inquiry: id
}

impl IOBluetoothDeviceInquiry{

    pub fn as_obj (&self) -> id {
        self.inquiry
    }

    /* It does not return already connected devices atm */
    pub fn new(delegate: id) -> IOBluetoothDeviceInquiry {
        let inquiry = unsafe {
            let inq: id = msg_send![Class::get("IOBluetoothDeviceInquiry").unwrap(), alloc];
            let () = msg_send![inq, initWithDelegate: delegate];
            let () = msg_send![inq, setUpdateNewDeviceNames: true];
            inq
        };
        IOBluetoothDeviceInquiry {
            inquiry
        }
    }

    // pub fn start(&self) {
    //     unsafe {
    //         let () = msg_send![self.inquiry, start];
    //     }
    // }

    // pub fn stop(&self) {
    //     unsafe {
    //         let () = msg_send![self.inquiry, stop];
    //     }
    // }

}