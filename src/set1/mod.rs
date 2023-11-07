use crate::simple_xor;
use crate::encoding::{Encoding, Hex, Base64};

use std::env::Args;
use std::fs;

pub fn challenge1(args: &mut Args) {

    if let Some(arg) = args.next() {
        let bytes = Hex::decode(arg.as_str()).unwrap();
        let hex_dec = Hex::encode(&bytes);
        let decoded = Base64::encode(&bytes);
        println!("Hex = {hex_dec}");
        println!("decoded = {decoded}");
    } else {
        panic!("Make sure to input a hexadecimal argument!");
    }

}

pub fn challenge2(args: &mut Args) {
    if let (Some(a1), Some(a2)) = (args.next(), args.next()) {
        let h1 = Hex::decode(a1.as_str()).unwrap();
        let h2 = Hex::decode(a2.as_str()).unwrap();

        let xored: Vec<_> =
            h1.iter().zip(h2).map(|(a, b)| a^b).collect();
        let hex = Hex::encode(&xored);

        println!("{hex}");
    } else {
        panic!("Make sure to input two hex strings!");
    }
}

pub fn challenge3(args: &mut Args) {
    if let Some(h) = args.next() {

        let h1 = Hex::decode(h.as_str()).unwrap();
        let (k, score) = simple_xor::best_key(h1.as_slice());
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
                let s = Hex::decode(s).unwrap();
                let (k, score) = simple_xor::best_key(s.as_slice());
                (idx, k, score, s)
            })
            .max_by(|(_, _, score1, _), (_, _, score2, _)| score1.cmp(score2))
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
        let key_bytes = key.as_bytes().iter();
        let contents = fs::read(fname).unwrap();
        let bytes = contents.as_slice().iter();
        
        let xored: Vec<_> = bytes.zip(key_bytes.cycle())
            .map(|(b, k)| b^k)
            .collect();
        let hex = Hex::encode(&xored);
        
        println!("{hex}");

    } else {
        panic!("Please enter an input");
    }
}
