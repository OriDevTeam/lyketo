// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use eyre::Result;
use crate::utils::four_cc::FourCC;


#[derive(Debug, Default)]
pub struct Tiger {}

#[allow(unused)]
impl Cipher for Tiger {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "Tiger";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }
}

