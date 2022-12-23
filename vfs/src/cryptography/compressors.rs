// Relative Modules
pub(crate) mod snappy;
pub(crate) mod lzo;
pub(crate) mod lzo_mini;
pub(crate) mod lzo_mini3;

// Standard Uses

// Crate Uses
// use crate::utils::four_cc;

// External Uses
use anyhow::Result;

// pub const MAGIC_MCOZ: u32 = four_cc::from_chars(['M', 'C', 'O', 'Z']);
pub const MAGIC_MCOZ: u32 = 1515144013;


pub trait Compressor {
    const FOURCC: u32;

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>>;
    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>>;
}


pub struct None;

#[allow(unused)]
impl Compressor for None {
    const FOURCC: u32 = 0;

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>> { unimplemented!(); }
    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>> { unimplemented!(); }
}

