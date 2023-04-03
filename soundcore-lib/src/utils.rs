use crate::error::SoundcoreError;

pub(crate) fn i8vec_to_u8vec(arr: Vec<i8>) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr {
        vec.push(i as u8);
    }
    vec
}

pub(crate) fn i8_to_u8vec(arr: &[i8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr.iter() {
        vec.push(*i as u8);
    }
    vec
}

pub(crate) fn u8_to_i8(val: u8) -> i8 {
    val as i8
}

pub(crate) fn mac_str_to_u64(addr: &str) -> Result<u64, std::num::ParseIntError> {
    let a = u64::from_str_radix(&addr.replace(':', ""), 16)?;
    Ok(a)
}

pub(crate) fn verify_resp(resp: &[u8]) -> Result<(), SoundcoreError> {
    if resp.is_empty() {
        return Err(SoundcoreError::ResponseChecksumError);
    }

    let len = resp.len() - 1;
    let mut b_arr2: Vec<u8> = vec![0; len];
    b_arr2.copy_from_slice(&resp[0..len]);

    match calculate_checksum_byte(&b_arr2) == resp[len] {
        true => Ok(()),
        false => Err(SoundcoreError::ResponseChecksumError),
    }
}

pub(crate) fn build_command_array_with_options_toggle_enabled(
    cmd: &[u8],
    optional_data: Option<&[u8]>,
) -> Vec<u8> {
    let length = cmd.len() + 2;
    let length2 = (if optional_data.is_some() {
        optional_data.unwrap().len()
    } else {
        0
    }) + length
        + 1;

    let mut output_arr = vec![0; length2 - 1];
    output_arr[..cmd.len()].copy_from_slice(cmd);
    let len_bytes = int_to_byte_array(length2 as i32);
    output_arr[cmd.len()] = len_bytes[3];
    output_arr[cmd.len() + 1] = len_bytes[2];
    if let Some(data) = optional_data {
        output_arr[length..].copy_from_slice(data);
    }

    calc_and_resize_checksum(&output_arr)
}

pub(crate) fn build_command_with_options(cmd: &[u8], optional_data: Option<&[u8]>) -> Vec<u8> {
    let length = cmd.len() + 2;
    let length2 = match optional_data {
        Some(data) => data.len() + length,
        None => 0,
    };

    let mut b_arr3 = Vec::with_capacity(length2);
    b_arr3.extend_from_slice(cmd);
    b_arr3.push((length2 % 256) as u8);
    b_arr3.push((length2 / 256) as u8);
    if optional_data.is_some() {
        b_arr3.extend_from_slice(optional_data.unwrap());
    }
    b_arr3
}

pub(crate) fn remove_padding(arr: &[u8]) -> Vec<u8> {
    /* Disable until we find a new method of removing the padded 0s from the end */
    // let mut out = arr
    //     .iter()
    //     .rev()
    //     .skip_while(|&&b| b == 0).copied()
    //     .collect::<Vec<u8>>();
    // out.reverse();
    // out
    arr.into()
}

pub(crate) fn int_to_byte_array(num: i32) -> [u8; 4] {
    // let bs: [u8; 4] = num.to_le_bytes();
    // bs
    [
        (num >> 24) as u8,
        (num >> 16) as u8,
        (num >> 8) as u8,
        num as u8,
    ]
}

pub(crate) fn calc_and_resize_checksum(cmd: &[u8]) -> Vec<u8> {
    let mut res = vec![0; cmd.len() + 1];
    res[..cmd.len()].copy_from_slice(cmd);
    res[cmd.len()] = calculate_checksum_byte(cmd);
    res
}

pub(crate) fn calculate_checksum_byte(cmd: &[u8]) -> u8 {
    if cmd.is_empty() {
        return 0;
    }
    let mut i = 0;
    for byte in cmd {
        i += *byte as i32;
    }
    (i & 0xFF).try_into().unwrap()
}

pub(crate) trait Clamp<T> {
    fn clamp(self, min: T, max: T) -> T;
}

impl<T> Clamp<T> for T
where
    T: PartialOrd + Copy,
{
    fn clamp(self, min: T, max: T) -> T {
        if self > max {
            max
        } else if self < min {
            min
        } else {
            self
        }
    }
}
