#[inline]
/// Perform a single byte xor.
pub fn single_byte_xor(array: &[u8], key: u8) -> Vec<u8> {
    array.iter().map(|x| x^key).collect()
}

#[inline]
pub fn multi_byte_xor(array: &[u8], key: &[u8]) -> Vec<u8> {
    array.iter().zip(key.iter().cycle()).map(|(b, k)| b^k).collect()
}
