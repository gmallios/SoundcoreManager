pub fn calculate_checksum_byte(data: &[u8]) -> u8 {
    data.iter().fold(0_u8, |acc, curr| acc.wrapping_add(*curr))
}
