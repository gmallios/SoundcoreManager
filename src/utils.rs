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

pub(crate) fn mac_str_to_u64(addr: &str) -> Result<u64, std::num::ParseIntError> {
    let a = u64::from_str_radix(&addr.replace(":", ""), 16)?;
    Ok(a)
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