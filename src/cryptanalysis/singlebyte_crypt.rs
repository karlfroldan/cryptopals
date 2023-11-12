use std::collections::HashMap;

use crate::simple_xor;
use phf::phf_map;

static FREQMAP: phf::Map<char, f64> = phf_map! {
    'e' => 11.1607,
    'a' => 8.4966,
    'r' => 7.5809,
    'i' => 7.5448,
    'o' => 7.1635,
    't' => 6.9509,
    'n' => 6.6544,
    's' => 5.7351,
    'l' => 5.4893,
    'c' => 4.5388,
    'u' => 3.6308,
    'd' => 3.3844,
    'p' => 3.1671,
    'm' => 3.0129,
    'h' => 3.0034,
    'g' => 2.4705,
    'b' => 2.072,
    'f' => 1.8121,
    'y' => 1.7779,
    'w' => 1.12899,
    'k' => 1.0074,
    'v' => 1.0074,
    'x' => 0.2902,
    'z' => 0.2722,
    'j' => 0.1965,
    'q' => 0.1962,
};

/// Return the key and the key score.
pub fn best_key(array: &[u8]) -> (u8, f64) {
    (u8::MIN..u8::MAX)
        .map(|k| (k, score(simple_xor::single_byte_xor(array, k).as_slice())))
        .filter(|(_, s)| s.clone() != 0.0)
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
}

/// Count the number of bytes that contains the ASCII
/// characters 'eatoin shrdlu'
pub fn score(array: &[u8]) -> f64 {
    let freq = histogram(array);
    chi_square(&freq)
}

fn chi_square(frequencies: &HashMap<u8, f64>) -> f64 {
    let terms  = ('a'..'z')
        .map(|c| {
            let ch = c as u8;

            let expected = FREQMAP.get(&c).unwrap().clone();
            let observed = frequencies.get(&ch).unwrap_or(&0.0).clone();

            f64::powi(observed - expected, 2) / expected
        });

    terms.sum()
}

fn histogram(arr: &[u8]) -> HashMap<u8, f64> {
    let size = arr.len() as f64;
    let arr = arr.iter()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_ascii());

    let mut hmap = HashMap::new();

    for c in arr {
        
        hmap.entry(c)
            .and_modify(|cntr| {
                *cntr += 1;
            })
            .or_insert(1);
    }

    // Convert to frequencies!!!
    hmap.into_iter()
        .map(|(c, f)| {
            (c, (f as f64) / size * 100.0)
        })
        .collect()
}
