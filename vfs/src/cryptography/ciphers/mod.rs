// Relative Modules
pub(crate) mod xtea;
pub(crate) mod sha1;
pub(crate) mod whirlpool;
pub(crate) mod tiger;
pub(crate) mod ripemd128;
pub(crate) mod tea;

// Standard Uses

// Crate Uses
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::Result;


pub trait Cipher {
    const FOURCC: FourCC;
    const NAME: &'static str;

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>>;
    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>>;
}


pub struct None;

impl Cipher for None {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "None";

    #[allow(unused)]
    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        // TODO; Decide what to really do here, give the same data back or panic
        Ok(data.to_vec())
    }

    #[allow(unused)]
    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        // TODO: Check note on the ENCRYPT function above, same applies here
        Ok(data.to_vec())
    }
}

