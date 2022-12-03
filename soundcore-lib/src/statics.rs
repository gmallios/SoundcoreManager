#![allow(warnings, unused)]
pub(crate) static RFCOMM_UUIDs: [&str; 4] = [
    "00001101-0000-1000-8000-00805F9B34FB", 
    "66666666-6666-6666-6666-666666666666",
    "77777777-7777-7777-7777-777777777777",
    "00002902-0000-1000-8000-00805f9b34fb",
];


pub(crate) static A3951_RFCOMM_UUID: &str = RFCOMM_UUIDs[0];
pub(crate) static A3951_CMD_DEVICE_STATUS: [i8; 7] = [8, -18, 0, 0, 0, 1, 1];
pub(crate) static A3951_CMD_DEVICE_INFO: [i8; 7] = [8, -18, 0, 0, 0, 1, 5];
pub(crate) static A3951_CMD_DEVICE_BATTERYLEVEL: [i8; 7] = [8, -18, 0, 0, 0, 1, 3];
pub(crate) static A3951_CMD_DEVICE_BATTERYCHARGING: [i8; 7] = [8, -18, 0, 0, 0, 1, 4];
pub(crate) static A3951_CMD_DEVICE_LDAC: [i8; 7] = [8, -18, 0, 0, 0, 1, 127]; // NOTE: Last byte is Byte.MAX_VALUE from java. Im not sure about the value
pub(crate) static A3951_CMD_DEVICE_GETEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, 1]; // Not tested yet.
pub(crate) static A3951_CMD_DEVICE_SETEQ_DRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -121]; // This gets used when DRC is supported/enabled.
pub(crate) static A3951_CMD_DEVICE_SETEQ_NODRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -122]; // This gets used when DRC is not supported/enabled.
pub(crate) static A3951_CMD_DEVICE_GETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, 1];
pub(crate) static A3951_CMD_DEVICE_SETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, -127];


