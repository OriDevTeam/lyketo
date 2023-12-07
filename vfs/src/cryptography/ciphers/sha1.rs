// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use eyre::Result;
use sha1::{Sha1, Digest};
use crate::utils::four_cc::FourCC;


#[derive(Debug, Default)]
pub struct SHA1 {}

#[allow(unused)]
impl Cipher for SHA1 {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "SHA1";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = Sha1::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = Sha1::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }
}

