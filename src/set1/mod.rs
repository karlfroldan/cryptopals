use crate::{simple_xor, cryptanalysis};
use crate::encoding::{Encoding, Decoding};
use crate::cryptanalysis::multibyte_crypt::{chunks_edit_distance, self};
use crate::cryptanalysis::singlebyte_crypt;
use crate::cipher::{traits::BlockCipher, block::Aes};

use itertools::Itertools;

use std::env::Args;
use std::fs;

pub fn challenge1(args: &mut Args) {

    if let Some(arg) = args.next() {
        
        println!("{}", arg.decode_hex().map(|v| v.encode_b64()).unwrap());
    } else {
        panic!("Make sure to input a hexadecimal argument!");
    }

}

pub fn challenge2(args: &mut Args) {
    if let (Some(a1), Some(a2)) = (args.next(), args.next()) {
        let h1 = a1.decode_hex().unwrap();
        let h2 = a2.decode_hex().unwrap();

        let xored: Vec<_> =
            h1.iter()
            .zip(h2).map(|(a, b)| a^b)
            .collect();
        let result = xored.encode_hex();

        println!("{result}");
    } else {
        panic!("Make sure to input two hex strings!");
    }
}

pub fn challenge3(args: &mut Args) {
    if let Some(h) = args.next() {
        let h1 = h.decode_hex().unwrap();
        let (k, score) = singlebyte_crypt::best_key(&h1);
        
        println!("Best key = {k:#x}, score = {score}");
        
        let plaintext = simple_xor::single_byte_xor(h1.as_slice(), k);
        let plaintext = String::from_utf8(plaintext).unwrap();
        println!("Plaintext message: {plaintext}");
    } else {
        panic!("Make sure to add an input hex!");
    }
}

pub fn challenge4(args: &mut Args) {
    if let Some(fname) = args.next() {
        let contents = fs::read_to_string(fname).unwrap();
        let (line_num, key, score, bs) = contents.clone().lines()
            .into_iter()
            .zip(0..)
            .map(|(s, idx)| {
                // Get all the best scores per line.
                let s = s.decode_hex().unwrap();
                let (k, score) = singlebyte_crypt::best_key(s.as_slice());
                (idx, k, score, s)
            })
            .max_by(|(_, _, score1, _), (_, _, score2, _)| score1.partial_cmp(score2).unwrap())
            .unwrap();

        println!("Score: {score}, key = {key}, line_number: {line_num}");
        let decrypted = simple_xor::single_byte_xor(bs.as_slice(), key);
        let decrypted = String::from_utf8(decrypted).unwrap();
        println!("{decrypted}");
    } else {
        panic!("Make sure to input the filename!");
    }
}

pub fn challenge5(args: &mut Args) {
    if let (Some(key), Some(fname)) = (args.next(), args.next()) {
        let contents = fs::read(fname).unwrap();
        let xored = simple_xor::multi_byte_xor(contents.as_slice(), key.as_bytes());
        println!("{}", xored.encode_hex());

    } else {
        panic!("Please enter an input");
    }
}

pub fn challenge6(args: &mut Args) {

    if let Some(fname) = args.next() {
        let contents = fs::read_to_string(fname).unwrap();
        let contents = contents.lines()
            .collect::<Vec<_>>()
            .concat();
        let input_contents = contents.decode_b64().unwrap();
        let keysize_limit = 40;

        // Find the best hamming distance.
        let distances = (2..keysize_limit + 1)
            .map(|keysize| {
                let a = input_contents.as_slice();
                (keysize, chunks_edit_distance(keysize, 8, a))
            })
            .sorted_by(|(_,d1), (_,d2)| d1.partial_cmp(d2).unwrap())
            .take(5); // Take the best 5 key sizes.

        let (_, key) = distances
            .map(|(keysize, _dist)| {
                let key = multibyte_crypt::find_key(keysize, input_contents.as_slice());
                let encrypted = input_contents.clone();
                let decrypted = simple_xor::multi_byte_xor(
                    encrypted.as_slice(),
                    key.as_slice(),
                );
                let score = singlebyte_crypt::score(decrypted.as_slice());
                (score, key)
            })
            .max_by(|(s1,_), (s2,_)| s1.cmp(s2)).unwrap();
        let key = String::from_utf8(key).unwrap();
        println!("key = {key}");

        let decrypted = simple_xor::multi_byte_xor(input_contents.as_slice(), key.as_bytes());
        let decrypted = String::from_utf8(decrypted).unwrap();
        println!("{decrypted}");
    } else {
        panic!("Please enter a filename");
    }
}

pub fn challenge7(args: &mut Args) {
    if let (Some(key), Some(fname)) = (args.next(), args.next()) {
        println!("key = {key}");
        println!("filename = {fname}");
        let encrypted = fs::read_to_string(fname).unwrap();
        let encrypted = encrypted.split("\n")
            .collect::<Vec<_>>()
            .concat()
            .decode_b64()
            .unwrap();
        
        let cipher = Aes::new();
        let decrypted = cipher.decrypt(key.as_bytes(), encrypted).unwrap();
        let decrypted = String::from_utf8(decrypted).unwrap();
        println!("{decrypted}");
    } else {
        panic!("Make sure to enter a key and a filename!!!");
    }
}

pub fn challenge8(args: &mut Args) {
    if let Some(fname) = args.next() {
        let file_contents = fs::read_to_string(fname).unwrap();
        let lines: Vec<_> = file_contents.split("\n")
            .map(|xs| xs.decode_b64().unwrap())
            .collect();

        let (line_num, score) = cryptanalysis::aes::find_ecb_mode(&lines);

        println!("Line {line_num} is encrypted with AES.");
        println!("There are {score} unique blocks");
        
    } else {
        panic!("Make sure to enter a key and a filename!!!");
    }
}
