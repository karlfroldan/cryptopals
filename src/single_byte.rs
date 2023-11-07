/// Count the number of bytes that contains the ASCII
/// characters 'eatoin shrdlu'
fn score(array: &[u8]) -> usize {
    let common = b"eatoinshrdlu";

    array.iter()
        .filter(|n| common.contains(n))
        .collect::<Vec<_>>()
        .len()
}

#[inline]
/// Perform a single byte xor.
pub fn single_byte_xor(array: &[u8], key: u8) -> Vec<u8> {
    array.iter().map(|x| x^key).collect()
}

pub fn best_key(array: &[u8]) -> (u8, usize) {
    (u8::MIN..u8::MAX)
        .map(|k| (k, score(single_byte_xor(array, k).as_slice())))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
}
