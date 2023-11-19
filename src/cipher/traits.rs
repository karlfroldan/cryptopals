use crate::common::Encrypted;

pub trait BlockCipher {
    fn decrypt<T: Encrypted>(&self, key: &[u8], input: T) -> Option<Vec<u8>>;
}
