use sha2::{Sha256, Sha512, Digest};

pub const SHA256_BYTES: usize = 32;
pub const SHA512_BYTES: usize = 64;

pub fn sha256(input: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

pub fn sha512(input: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}
