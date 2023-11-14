use super::singlebyte_crypt;
use crate::common::Encrypted;

pub fn hamming_distance<T: Encrypted>(a: T, b: T) -> u32 {
    let a = a.as_encrypted_slice();
    let b = b.as_encrypted_slice();
    
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x^y).count_ones())
        .sum()
}

pub fn chunks_edit_distance<T: Encrypted>(chunksize: usize,
                                          num_chunks: usize,
                                          a: T) -> f64 {
    let mut rest = a.as_encrypted_slice();
    let f_keysize = chunksize as f64;

    (0..num_chunks)
        .map(|n| {
            let (chunk_group, rest1) = rest.split_at(chunksize * 2);
            let (chunk1, chunk2) = chunk_group.split_at(chunksize);

            rest = rest1;
            
            hamming_distance(chunk1, chunk2) as f64 / f_keysize
        })
        .sum::<f64>() as f64 / num_chunks as f64
}

pub fn find_key<T: Encrypted>(keysize: usize, slice: T) -> Vec<u8> {
    let slice = slice.as_encrypted_slice();
    (0..keysize)
        .map(|i| {
            let slice = slice.clone();
            let transposed: Vec<_> = slice.iter()
                .skip(i) // start iterating from the i-th character.
                .step_by(keysize)
                .map(|x| x.clone()) // Ensure Vec<u8>, not Vec<&u8>
                .collect();

            singlebyte_crypt::best_key(transposed.as_slice()).0
        }).collect()
}
