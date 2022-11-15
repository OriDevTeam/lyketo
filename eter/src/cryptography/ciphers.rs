// Relative Modules
pub(crate) mod xtea;
pub(crate) mod sha1;
pub(crate) mod whirlpool;
pub(crate) mod tiger;
pub(crate) mod ripemd128;
pub(crate) mod tea;

// Standard Uses

// Crate Uses

// External Uses
use anyhow::Result;


pub trait Cipher {
    fn encrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>>;
    fn decrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>>;
}


pub struct None;

#[allow(unused)]
impl Cipher for None {
    fn encrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> { unimplemented!() }

    fn decrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> { unimplemented!() }
}
