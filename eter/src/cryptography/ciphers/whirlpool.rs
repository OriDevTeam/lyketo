// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use whirlpool::{Whirlpool, Digest};


#[derive(Debug, Default)]
pub struct Whirpool {}

impl Cipher for Whirpool {
    fn encrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let mut hasher = Whirlpool::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }

    fn decrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let mut hasher = Whirlpool::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }
}

