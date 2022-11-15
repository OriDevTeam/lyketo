// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use anyhow::Result;


#[derive(Debug, Default)]
pub struct Tiger {}

#[allow(unused)]
impl Cipher for Tiger {
    fn encrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }

    fn decrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }
}

