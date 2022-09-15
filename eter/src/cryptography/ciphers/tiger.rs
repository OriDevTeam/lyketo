// Standard Uses

// Crate Uses

// External Uses
use crate::cryptography::ciphers::Cipher;


#[derive(Debug, Default)]
pub struct Tiger {}

impl Cipher for Tiger {
    fn encrypt(_data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }

    fn decrypt(_data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }
}

