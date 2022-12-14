// Standard Uses

// Crate Uses
use crate::cryptography::compressors::{Compressor, MAGIC_MCOZ};

// External Uses
use anyhow::Result;
use minilzo3;


pub struct LZOMini3 {}
impl Compressor for LZOMini3 {
    const FOURCC: u32 = MAGIC_MCOZ;

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>> {
        let mut out = vec![0u8; data.len()];
        minilzo3::compress(&data, &mut *out, true).unwrap();

        Ok(out)
    }

    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>> {
        let mut out = vec![0u8; expected_size];
        minilzo3::decompress(&data, &mut out).unwrap();

        Ok(out.to_vec())
    }
}

