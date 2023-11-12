use itertools::Itertools;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x^y).count_ones())
        .sum()
}

pub fn chunks_edit_distance(keysize: u32, _num_chunks: usize, a: &[u8]) -> f64 {
    let keysize_big = keysize as usize;
    let (chunk1, rest) = a.split_at(keysize_big);
    let (chunk2, rest) = rest.split_at(keysize_big);
    let (chunk3, rest) = rest.split_at(keysize_big);
    let (chunk4, _) = rest.split_at(keysize_big);

    let f_keysize = f64::from(keysize);

    let hamming1 = f64::from(hamming_distance(chunk1, chunk2));
    let hamming2 = f64::from(hamming_distance(chunk3, chunk4));

    (hamming1 + hamming2) / f_keysize
}
