// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use eyre::Result;
use ripemd128::{Digest, Ripemd128};
use crate::utils::four_cc::FourCC;


#[derive(Debug, Default)]
pub struct RIPEMD128 {}

#[allow(unused)]
impl Cipher for RIPEMD128 {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "RIPEMD128";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = Ripemd128::new();
        hasher.input(data);

        Ok(hasher.result().to_vec())
    }

    #[allow(unused)]
    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }
}

