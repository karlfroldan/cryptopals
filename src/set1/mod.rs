use crate::simple_xor;
use crate::encoding::{Encoding, Decoding, EncodingType, base64::Base64, hex::Hex};
use crate::cryptanalysis::multibyte_crypt::chunks_edit_distance;
use crate::cryptanalysis::singlebyte_crypt;

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
        let (k, score) = singlebyte_crypt::best_key(h1.as_slice());
        
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
            .min_by(|(_, _, score1, _), (_, _, score2, _)| score1.partial_cmp(score2).unwrap())
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
                (keysize, chunks_edit_distance(keysize, 4, a))
            })
            .sorted_by(|(_,d1), (_,d2)| d1.partial_cmp(d2).unwrap())
            .take(1); // Take the best 5 keys.
        println!("input_contents: {input_contents:?}\n");


        for (keysize, dist) in distances {
            println!("Breaking keysize {keysize}");
            // println!("input_contents: {input_contents:?}");

            let key: Vec<_> = (0..keysize)
                .map(|i| {
                    let input_contents = input_contents.clone();
                    let transposed: Vec<_> = input_contents.iter()
                        .skip(i as usize) // start iterating from the i-th character.
                        .step_by(keysize as usize)
                        .map(|x| x.clone()) // Make sure the type is Vec<u8>
                        .collect();

                    println!("transposed: {transposed:?}");
                    
                    // Return the best character for this chunk.
                    singlebyte_crypt::best_key(transposed.as_slice()).0
                })
                .collect();
            //let key = String::from_utf8(best_keys).unwrap();

            println!("Got Possible key = \"{key:?}\"");

            // Encrypt!!!
            let encrypted = input_contents.clone();
            let decrypted = simple_xor::multi_byte_xor(
                encrypted.as_slice(),
                key.as_slice());
            let score = singlebyte_crypt::score(decrypted.as_slice());
            println!("Keysize {keysize} has score {score}");

            let key = String::from_utf8(key).unwrap();

            println!("key = {key}");

            let decrypted = String::from_utf8(decrypted).unwrap();
            //println!("decrypted = {decrypted}");
        }
    } else {
        panic!("Please enter a filename");
    }
}
