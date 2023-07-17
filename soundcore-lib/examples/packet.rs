use soundcore_lib::api::SoundMode;

fn main() {
    let a = SoundMode::NormalMode;
    println!("a: {:?}", a);
    println!(
        "a: {:?}",
        SoundMode::NoiseCancelling(soundcore_lib::api::ANCModes::Indoor)
    );

    let packet = [8, -18, 0, 0, 0, 6, -127];
    let bytes = i8_to_u8vec(&packet);
    println!("bytes: {:X?}", bytes);
    let bytes2 = build_command_array_with_options_toggle_enabled(&bytes, Some(&[1, 2, 3, 4]));
    println!("bytes2: {:X?}", bytes2);
}

fn i8_to_u8vec(arr: &[i8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in arr.iter() {
        vec.push(*i as u8);
    }
    vec
}

fn build_command_array_with_options_toggle_enabled(
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
    println!("len2: {}", length2);
    let len_bytes = int_to_byte_array(length2 as i32);
    output_arr[cmd.len()] = len_bytes[3];
    output_arr[cmd.len() + 1] = len_bytes[2];
    if let Some(data) = optional_data {
        output_arr[length..].copy_from_slice(data);
    }

    output_arr.to_vec()
}

fn int_to_byte_array(num: i32) -> [u8; 4] {
    // let bs: [u8; 4] = num.to_le_bytes();
    // bs
    [
        (num >> 24) as u8,
        (num >> 16) as u8,
        (num >> 8) as u8,
        num as u8,
    ]
}
