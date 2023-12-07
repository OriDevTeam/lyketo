// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use eyre::{Result};
use whirlpool::{Whirlpool as WhirlpoolWrapper, Digest};
use crate::utils::four_cc::FourCC;


#[derive(Debug, Default)]
pub struct Whirlpool {}

#[allow(unused)]
impl Cipher for Whirlpool {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "Whirlpool";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = WhirlpoolWrapper::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        let mut hasher = WhirlpoolWrapper::new();
        hasher.update(data);

        Ok(hasher.finalize().to_vec())
    }
}

