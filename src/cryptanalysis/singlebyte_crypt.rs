use crate::simple_xor;
use crate::common::Encrypted;

pub fn score<T: Encrypted>(slice: T) -> usize {
    let slice = slice.as_encrypted_slice();
    let most_freq = "eatoin shrdlu".as_bytes();
    slice.iter().filter(|ch| most_freq.contains(&ch)).count()
}

pub fn best_key<T: Encrypted>(slice: T) -> (u8, usize) {
    let slice = slice.as_encrypted_slice();
    
    // Get the scores of all possible keys.
    (u8::MIN + 1..u8::MAX)
        .map(|k| {
            let xored = simple_xor::single_byte_xor(slice, k);
            (k, score(xored.as_slice()))
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()    
}
