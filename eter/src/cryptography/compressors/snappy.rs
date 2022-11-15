// Standard Uses

// Crate Uses
use crate::cryptography::compressors::{Compressor, MAGIC_MCOZ};


// External Uses
use anyhow::Result;


#[derive(Default)]
pub struct Snappy {}


#[allow(unused)]
impl Compressor for Snappy {
    const FOURCC: u32 = MAGIC_MCOZ;

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>> {
        todo!()
    }

    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>> {
        todo!()
    }
}
