use crate::common::Encrypted;
use crate::cipher::traits::BlockCipher;
use crate::cipher::block::Block128;

pub struct AES128 {
    state: Block128,
}

impl AES128 {
    pub fn new() -> Self {
        Self {
            state: Block128::new(&[0;16]),
        }
    }
}

impl BlockCipher for AES128 {
    fn decrypt<T: Encrypted>(self: &Self, key: &[u8], input: T) -> Option<Vec<u8>> {
        let ciphertext = input.as_encrypted_slice();
        let mut idx = 0;
        let mut block = Block128::new(&ciphertext[idx..]);
        
        Some(vec![1,2,3])
    }
}
