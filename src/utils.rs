pub fn i8vec_to_u8vec(arr: Vec<i8>) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr {
        vec.push(i as u8);
    }
    return vec;
}

pub fn i8_to_u8vec(arr: &[i8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr.iter() {
        vec.push(*i as u8);
    }
    return vec;
}