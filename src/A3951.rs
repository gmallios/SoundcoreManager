use crate::utils::i8_to_u8vec;

// LIBERTY AIR 2 PRO

pub static CMD_DEVICE_INFO: [i8; 7] = [8,-18,0,0,0,1,1];


pub fn create_A3951_command(inp: [i8; 7]) -> Vec<u8>{
    return i8_to_u8vec(&inp);
}

pub trait A3951 {
    fn send_command(&mut self, command: Vec<u8>);
    fn parse_info_response(&self, response: Vec<u8>) -> String;
}

struct A3951_STATUS {
    HOST_DEVICE: u8,
    TWS_STATUS: bool,
    LEFT_BATTERY: u8,
    RIGHT_BATTERY: u8,
    LEFT_CHARGING: bool,
    RIGHT_CHARGING: bool

}

struct A3951_ANC {
    
}

