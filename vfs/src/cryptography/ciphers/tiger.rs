// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use anyhow::Result;


#[derive(Debug, Default)]
pub struct Tiger {}

#[allow(unused)]
impl Cipher for Tiger {
    const NAME: &'static str = "Tiger";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }
}

