// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use anyhow::Result;
use sha1::{Sha1, Digest};


#[derive(Debug, Default)]
pub struct SHA1 {}

#[allow(unused)]
impl Cipher for SHA1 {
    fn encrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = Sha1::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }

    fn decrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = Sha1::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }
}

