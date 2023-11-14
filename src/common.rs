pub trait Encrypted {
    fn as_encrypted_slice(self: &Self) -> &[u8];
}

impl Encrypted for &[u8] {
    fn as_encrypted_slice(self: &Self) -> &[u8] {
        self
    }
}

impl Encrypted for Vec<u8> {
    fn as_encrypted_slice(self: &Self) -> &[u8] {
        self.as_slice()
    }
}

impl Encrypted for &Vec<u8> {
    fn as_encrypted_slice(self: &Self) -> &[u8] {
        self.as_slice()
    }
}

impl Encrypted for String {
    fn as_encrypted_slice(self: &Self) -> &[u8] {
        self.as_bytes()
    }
}

impl Encrypted for &String {
    fn as_encrypted_slice(self: &Self) -> &[u8] {
        self.as_bytes()
    }
}
