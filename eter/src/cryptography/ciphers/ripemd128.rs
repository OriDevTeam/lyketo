// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use ripemd128::{Digest, Ripemd128};


#[derive(Debug, Default)]
pub struct RIPEMD128 {}

impl Cipher for RIPEMD128 {
    fn encrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let mut hasher = Ripemd128::new();
        hasher.input(data);

        Ok(hasher.result().to_vec())
    }

    fn decrypt(_data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }
}

