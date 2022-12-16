use cocoa::base::id;
use futures::channel::mpsc::{self, Receiver, Sender};
use futures::sink::SinkExt;
use libc::c_void;
use objc::runtime::Class;
use objc::{
    class,
    declare::ClassDecl,
    runtime::{Object, Sel},
};
use objc::{msg_send, sel, sel_impl};
use std::sync::Once;

/* Huge Thanks to https://github.com/deviceplug/btleplug/blob/886a905298d007fb4a53f6490b8e35f110a1c2c1/src/corebluetooth/central_delegate.rs */

pub mod InquiryDelegate {
    use cocoa::base::BOOL;
    use core_foundation::runloop::{CFRunLoopGetCurrent, CFRunLoopStop};
    use log::trace;

    pub type AddDeviceCallback = fn(IOBTDevice) -> ();

    use crate::iobluetoothdevice::IOBTDevice;

    use super::*;

    const DELEGATE_SENDER_IVAR: &str = "_add_dev_cb";

    pub fn delegate(cb: AddDeviceCallback) -> id {
        let cb = Box::new(cb);
        let delegate = unsafe {
            let mut delegate: id = msg_send![delegate_class(), alloc];
            delegate = msg_send![delegate, initWithSender: Box::into_raw(cb) as *mut c_void];
            delegate
        };
        delegate
    }

    fn delegate_class() -> &'static Class {
        static REGISTER_DELEGATE_CLASS: Once = Once::new();
        REGISTER_DELEGATE_CLASS.call_once(|| {
            let mut decl = ClassDecl::new("IOBTInquiryDelegate", class!(NSObject)).unwrap();
            // For some odd reason IOBluetoothDeviceInquiryDelegate is not found, but using only NSObject works
            //let proto = Protocol::get("IOBluetoothDeviceInquiryDelegate").unwrap();
            //decl.add_protocol(proto);
            decl.add_ivar::<*mut c_void>(DELEGATE_SENDER_IVAR);
            unsafe {
                decl.add_method(
                    sel!(initWithSender:),
                    delegate_init as extern "C" fn(&mut Object, Sel, *mut c_void) -> id,
                );
                decl.add_method(
                    sel!(deviceInquiryComplete:error:aborted:),
                    inquirydelegate_deviceInquiryComplete
                        as extern "C" fn(&mut Object, Sel, *mut Object, i32, BOOL),
                );
                decl.add_method(
                    sel!(deviceInquiryStarted:),
                    inquirydelegate_deviceInquiryStarted
                        as extern "C" fn(&mut Object, Sel, *mut Object),
                );
                decl.add_method(
                    sel!(deviceInquiryDeviceFound:device:),
                    inquirydelegate_deviceInquiryDeviceFound
                        as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object),
                );
            }
            decl.register();
        });

        class!(IOBTInquiryDelegate)
    }

    /* Utility Functions */

    fn delegate_get_cb_fn(delegate: &mut Object) -> AddDeviceCallback {
        unsafe {
            let cb = &(*(*(&*delegate).get_ivar::<*mut c_void>(DELEGATE_SENDER_IVAR)
                as *mut AddDeviceCallback));
            cb.clone()
        }
    }

    extern "C" fn delegate_init(delegate: &mut Object, _cmd: Sel, sender: *mut c_void) -> id {
        /* initWithSender */
        unsafe {
            delegate.set_ivar(DELEGATE_SENDER_IVAR, sender);
        }
        delegate
    }

    /* Handlers */

    extern "C" fn inquirydelegate_deviceInquiryStarted(
        _delegate: &mut Object,
        _cmd: Sel,
        _sender: *mut Object,
    ) {
        trace!("inquiry started");
    }

    extern "C" fn inquirydelegate_deviceInquiryComplete(
        _delegate: &mut Object,
        _cmd: Sel,
        _sender: *mut Object,
        _error: i32,
        _aborted: BOOL,
    ) {
        unsafe {
            CFRunLoopStop(CFRunLoopGetCurrent());
        }
        trace!("inquiry complete");
    }

    extern "C" fn inquirydelegate_deviceInquiryDeviceFound(
        delegate: &mut Object,
        _cmd: Sel,
        _sender: *mut Object,
        device: *mut Object,
    ) {
        unsafe {
            trace!("device found");
            delegate_get_cb_fn(delegate)(IOBTDevice::from_id(device));
        }
    }
}
