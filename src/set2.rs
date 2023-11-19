use std::env::Args;
use std::fs;

use crate::cipher::block::pkcs7;

pub fn challenge9(args: &mut Args) {
    if let (Some(arg), Some(padsize)) = (args.next(), args.next()) {
        let padsize = padsize.parse().unwrap();
        println!("arg: {arg}");

        println!("result: {:#x?}", pkcs7(arg.as_bytes(), padsize));
    } else {
        panic!("Usage (two arguments): string and padsize (usize)");
    }
}
