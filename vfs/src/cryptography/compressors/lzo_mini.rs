// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;

// External Uses
use anyhow::Result;
use minilzo;


pub struct LZOMini {}

impl Compressor for LZOMini {
    //const FOURCC: u32 = MAGIC_MCOZ;
    const NAME: &'static str = "LZOMini";

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let compressed = minilzo::compress(&data).unwrap();

        Ok(compressed)
    }

    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        let decompressed = minilzo::decompress(&data, expected_size).unwrap();

        Ok(decompressed)
    }
}

