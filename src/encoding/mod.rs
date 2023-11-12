pub mod hex;
pub mod base64;

use crate::encoding::{hex::Hex, base64::Base64};

pub trait EncodingType {
    fn decode(input: &str) -> Option<Vec<u8>>;
    fn encode(input: &Vec<u8>) -> String;
}

/// Trait for encoding different data types.
pub trait Encoding {
    fn encode_hex(self: &Self) -> String;
    fn encode_b64(self: &Self) -> String;
}

pub trait Decoding {
    fn decode_hex(self: Self) -> Option<Vec<u8>>;
    fn decode_b64(self: Self) -> Option<Vec<u8>>;
}

impl Encoding for Vec<u8> {
    fn encode_hex(self: &Self) -> String {
        Hex::encode(self)
    }


    fn encode_b64(self: &Self) -> String {
        Base64::encode(self)
    }
}

impl Decoding for &str {
    fn decode_hex(self: Self) -> Option<Vec<u8>> {
        Hex::decode(self)
    }
    fn decode_b64(self: Self) -> Option<Vec<u8>> {
        Base64::decode(self)
    }
   
}

impl Decoding for String {
    fn decode_hex(self: Self) -> Option<Vec<u8>> {
        self.as_str().decode_hex()
    }

    fn decode_b64(self: Self) -> Option<Vec<u8>> {
        self.as_str().decode_b64()
    }
}

impl Decoding for &String {
    fn decode_hex(self: Self) -> Option<Vec<u8>> {
        self.as_str().decode_hex()
    }

    fn decode_b64(self: Self) -> Option<Vec<u8>> {
        self.as_str().decode_b64()
    }
}
