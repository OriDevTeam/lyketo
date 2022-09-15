// Relative Modules
pub(crate) mod xtea;
pub(crate) mod sha1;
pub(crate) mod whirlpool;
pub(crate) mod tiger;
pub(crate) mod ripemd128;

// Standard Uses

// Crate Uses

// External Uses


pub trait Cipher {
    fn encrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()>;
    fn decrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()>;
}


pub struct None;

impl Cipher for None {
    fn encrypt(_data: &Vec<u8>) -> Result<Vec<u8>, ()> { unimplemented!() }

    fn decrypt(_data: &Vec<u8>) -> Result<Vec<u8>, ()> { unimplemented!() }
}
