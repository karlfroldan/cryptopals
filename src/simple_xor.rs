use crate::common::Encrypted;

#[inline]
/// Perform a single byte xor.
pub fn single_byte_xor<T: Encrypted>(array: T, key: u8) -> Vec<u8> {
    array.as_encrypted_slice().iter().map(|x| x^key).collect()
}

#[inline]
pub fn multi_byte_xor<T: Encrypted>(array: T, key: &[u8]) -> Vec<u8> {
    array.as_encrypted_slice()
        .iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b^k)
        .collect()
}
