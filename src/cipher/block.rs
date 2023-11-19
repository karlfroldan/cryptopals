use openssl::symm;

use crate::cipher::traits::BlockCipher;
use crate::common::Encrypted;

#[derive(Debug)]
pub struct Block128 {
    block: [[u8; 4]; 4],
}

impl Block128 {
    pub fn new(input: &[u8]) -> Self {
        let mut b = Self {
            block: [[0; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                // Store in column major order.
                let k = (4 * i) + j;
                b.block[i][j] = input[k];
            }
        }

        b
    }
}


pub struct Aes {
    cipher: symm::Cipher,
}

impl Aes {
    pub fn new() -> Self {
        Self {
            cipher: symm::Cipher::aes_128_ecb(),
        }
    }
}

impl BlockCipher for Aes {
    fn decrypt<T: Encrypted>(self: &Self, key: &[u8], input: T) -> Option<Vec<u8>> {
        let ciphertext = input.as_encrypted_slice();
        let mut crypter = symm::Crypter::new(self.cipher,
                                             symm::Mode::Decrypt,
                                             key,
                                             None).ok()?;

        crypter.pad(false);

        let block_size = self.cipher.block_size();
        let mut plaintext = vec![0; ciphertext.len() + block_size];
        let mut count = crypter.update(
            &ciphertext,
            &mut plaintext).unwrap();
        count += crypter.finalize(&mut plaintext[count..]).unwrap();
        Some(plaintext)
    }
}
