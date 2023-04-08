#![allow(warnings, unused)]
pub(crate) static A3951_RESPONSE_VERIFICATION: bool = false;
pub(crate) static A3935_RESPONSE_VERIFICATION: bool = false;
pub(crate) static A3040_RESPONSE_VERIFICATION: bool = false;
pub(crate) static A3027_RESPONSE_VERIFICATION: bool = false;

pub(crate) static EQ_INDEX_DEFAULT: i32 = 0;
pub(crate) static EQ_INDEX_CUSTOM: i32 = 65278;

pub(crate) static A3951_RFCOMM_UUID: &str = "00001101-0000-1000-8000-00805F9B34FB";
pub(crate) static A3951_CMD_DEVICE_STATUS: [i8; 7] = [8, -18, 0, 0, 0, 1, 1];
pub(crate) static A3951_CMD_DEVICE_INFO: [i8; 7] = [8, -18, 0, 0, 0, 1, 5];
pub(crate) static A3951_CMD_DEVICE_BATTERYLEVEL: [i8; 7] = [8, -18, 0, 0, 0, 1, 3];
pub(crate) static A3951_CMD_DEVICE_BATTERYCHARGING: [i8; 7] = [8, -18, 0, 0, 0, 1, 4];
pub(crate) static A3951_CMD_DEVICE_GETLDAC: [i8; 7] = [8, -18, 0, 0, 0, 1, 127]; // NOTE: Last byte is Byte.MAX_VALUE from java. Im not sure about the value
pub(crate) static A3951_CMD_DEVICE_GETEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, 1]; // Not tested yet.
pub(crate) static A3951_CMD_DEVICE_SETEQ_DRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -121]; // This gets used when DRC is supported/enabled.
pub(crate) static A3951_CMD_DEVICE_SETEQ_NODRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -122]; // This gets used when DRC is not supported/enabled.
pub(crate) static A3951_CMD_DEVICE_GETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, 1];
pub(crate) static A3951_CMD_DEVICE_SETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, -127];

pub(crate) static A3027_CMD_DEVICE_SETEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, -127];

pub(crate) static A3040_RFCOMM_UUID: &str = "0CF12D31-FAC3-4553-BD80-D6832E7B3040";
pub(crate) static A3040_CMD_DEVICE_INFO: [i8; 7] = [8, -18, 0, 0, 0, 1, 1];
pub(crate) static A3040_CMD_DEVICE_SETANC: [i8; 7] = A3951_CMD_DEVICE_SETANC;
pub(crate) static A3040_CMD_DEVICE_BATTERYLEVEL: [i8; 7] = A3951_CMD_DEVICE_BATTERYLEVEL;
pub(crate) static A3040_CMD_DEVICE_CHARGINSTATUS: [i8; 7] = A3951_CMD_DEVICE_BATTERYCHARGING;
pub(crate) static A3040_CMD_DEVICE_SETCUSTOMEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, -125];
pub(crate) static A3040_CMD_DEVICE_SETLDAC: [i8; 7] = [8, -18, 0, 0, 0, 1, -1];
