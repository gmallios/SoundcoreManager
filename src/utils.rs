pub(crate) fn i8vec_to_u8vec(arr: Vec<i8>) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr {
        vec.push(i as u8);
    }
    return vec;
}

pub(crate) fn i8_to_u8vec(arr: &[i8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr.iter() {
        vec.push(*i as u8);
    }
    return vec;
}

pub(crate) fn u8_to_i8(val: u8) -> i8 {
    return val as i8;
}

pub(crate) fn mac_str_to_u64(addr: &str) -> Result<u64, std::num::ParseIntError> {
    let a = u64::from_str_radix(&addr.replace(":", ""), 16)?;
    Ok(a)
}


pub(crate) fn build_command_array_with_options_toggle_enabled(
    bArr: &[u8],
    bArr2: Option<&[u8]>,
) -> Vec<u8> {
    let length = bArr.len() + 2;
    let length2 = (if bArr2.is_some() {
        bArr2.unwrap().len()
    } else {
        0
    }) + length
        + 1;

    let mut bArr3 = vec![0; length2 - 1];
    bArr3[..bArr.len()].copy_from_slice(bArr);
    let len2bArr = int_to_byte_array(length2 as i32);
    bArr3[bArr.len()] = len2bArr[3] & 0xFF;
    bArr3[bArr.len() + 1] = len2bArr[2] & 0xFF;
    if bArr2.is_some() {
        bArr3[..length].copy_from_slice(&bArr2.unwrap());
    }

    return calculate_checksum(&bArr3);
}

pub(crate) fn int_to_byte_array(num: i32) -> [u8; 4] {
    // let bs: [u8; 4] = num.to_le_bytes();
    // bs
    [
        (num >> 24) as u8 & 0xff,
        (num >> 16) as u8 & 0xff,
        (num >> 8) as u8 & 0xff,
        num as u8 & 0xff,
    ]
}

pub(crate) fn calculate_checksum(bArr: &[u8]) -> Vec<u8> {
    let mut res = vec![0; bArr.len() + 1];
    res[..bArr.len()].copy_from_slice(bArr);
    res[bArr.len()] = calculate_checksum_byte(bArr);
    return res;
}

pub fn calculate_checksum_byte(bArr: &[u8]) -> u8 {
    if (bArr.is_empty()) {
        return 0;
    }
    let mut i = 0;
    for byte in bArr {
        i += (byte & 0xFF) as i32;
    }
    return (i & 0xFF).try_into().unwrap();
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
