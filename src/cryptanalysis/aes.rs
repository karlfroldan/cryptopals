use std::collections::HashSet;

use crate::common::Encrypted;

fn count_similar_blocks<T: Encrypted>(ciphertext: T) -> usize {
    let mut hset = HashSet::new();
    let ciphertext = ciphertext.as_encrypted_slice();

    let mut i = 0;

    while i < ciphertext.len() {
        let mut cipher_slice = &ciphertext[i..i+16];
        hset.insert(cipher_slice);
        i += 16;
    }

    hset.len()
}

/// Detect ECB mode in AES. Return line number and number of unique blocks.
pub fn find_ecb_mode<T: Encrypted + Clone>(list: &Vec<T>) -> (usize, usize) {
    let mut i = 0;
    let mut score = usize::MAX;
    let mut best_line_num = 0;
    for line in list {
        let n = count_similar_blocks(line.clone());
        if n < score && n != 0 {
            score = n;
            best_line_num = i;
        }
        i += 1;
    }

    (best_line_num, score)
}
