use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Once,
};

use cocoa::base::id;
use libc::{c_void, size_t};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};

use crate::util::IOBluetoothRFCOMMChannel;

pub type RfcommDelegateOnDataRecv = fn(data: &[u8]);

const DELEGATE_RECV_CB_IVAR: &str = "recv_cb";

pub fn delegate(on_data: RfcommDelegateOnDataRecv) -> id {
    let cb = Box::new(on_data);
    let delegate = unsafe {
        let delegate: id = msg_send![delegate_class(), alloc];
        let () = msg_send![delegate, initWithCB: Box::into_raw(cb) as *mut c_void];
        delegate
    };
    delegate
}

fn delegate_class() -> &'static Class {
    static REGISTER_DELEGATE_CLASS: Once = Once::new();
    REGISTER_DELEGATE_CLASS.call_once(|| {
        let mut decl = ClassDecl::new("RFDelegate", class!(NSObject)).unwrap();
        decl.add_ivar::<*mut c_void>(DELEGATE_RECV_CB_IVAR);
        unsafe {
            decl.add_method(
                sel!(initWithCB:),
                rfcommdelegate_initWithCB as extern "C" fn(&mut Object, Sel, *mut c_void) -> id,
            );
            decl.add_method(
                sel!(rfcommChannelOpenComplete:status:),
                rfcommdelegate_rfcommChannelOpenComplete
                    as extern "C" fn(&mut Object, Sel, *mut Object, i32),
            );
            decl.add_method(
                sel!(rfcommChannelWriteComplete:refcon:status:bytesWritten:),
                rfcommdelegate_rfcommChannelWriteComplete
                    as extern "C" fn(&mut Object, Sel, *mut Object, *mut c_void, i32, size_t),
            );
            decl.add_method(
                sel!(rfcommChannelData:data:length:),
                rfcommdelegate_rfcommChannelData
                    as extern "C" fn(&mut Object, Sel, *mut Object, *mut c_void, size_t),
            );
        }
        decl.register();
    });
    class!(RFDelegate)
}

extern "C" fn rfcommdelegate_initWithCB(delegate: &mut Object, _cmd: Sel, cb: *mut c_void) -> id {
    unsafe {
        delegate.set_ivar(DELEGATE_RECV_CB_IVAR, cb);
    }
    delegate
}

fn delegate_get_cb_fn(delegate: &mut Object) -> RfcommDelegateOnDataRecv {
    unsafe {
        let cb = &(*(*(&*delegate).get_ivar::<*mut c_void>(DELEGATE_RECV_CB_IVAR)
            as *mut RfcommDelegateOnDataRecv));
        cb.clone()
    }
}

extern "C" fn rfcommdelegate_rfcommChannelData(
    delegate: &mut Object,
    _cmd: Sel,
    _channel: *mut Object,
    data: *mut c_void,
    length: size_t,
) {
    // Data is u8 array
    let data = unsafe { std::slice::from_raw_parts(data as *const u8, length) };
    println!("rfcommChannelData, data: {:?}", data);
    delegate_get_cb_fn(delegate)(data);
}

extern "C" fn rfcommdelegate_rfcommChannelWriteComplete(
    _delegate: &mut Object,
    _cmd: Sel,
    channel: *mut Object,
    refcon: *mut c_void,
    status: i32,
    bytes_written: size_t,
) {
    println!("rfcommChannelWriteComplete, len: {}", bytes_written);
}

extern "C" fn rfcommdelegate_rfcommChannelOpenComplete(
    _delegate: &mut Object,
    _cmd: Sel,
    channel: *mut Object,
    status: i32,
) {
    println!("rfcommChannelOpenComplete");
    println!("channel_from_delegate: {:?}", channel);
}
