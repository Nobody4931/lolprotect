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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_returns_correct_digest() {
        assert_eq!(sha256("sample input"), hex::decode("78fe461fd72e68a1c7922b227a462e4b417e58bbccc81c6986d863069d8ae74e").unwrap());
        assert_eq!(sha256("another input"), hex::decode("9779765541c59829a4e2196d1f3de1e33ac57753ef039737273b11af7734d7ac").unwrap());
    }

    #[test]
    fn sha512_returns_correct_digest() {
        assert_eq!(sha512("sample input"), hex::decode("879281e1bcc1082c51bb925b37c7dbd8cd427bb980dba905bb3d42717202ec574e03120495aef6ba746b338b17c0ec09e7168dfdd55f1084aa0fa23f71672ffe").unwrap());
        assert_eq!(sha512("another input"), hex::decode("797dd10db46766aae7dc0291c65c6ef10c8e2402467fead0d0af969ffc9c7de9db69bfa51925b50c73a4feb938078575bf4d03077d27c186a5c6637b4d6a5e03").unwrap());
    }
}
