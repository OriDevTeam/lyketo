// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;

// External Uses
use minilzo;
use minilzo3;
use rust_lzo::LZOContext;


#[derive(Default)]
pub struct LZO {}

impl Compressor for LZO {
    const FOURCC: &'static str = "MCOZ";

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let compressed = minilzo3::compress_vec(&data, true).unwrap();

        Ok(compressed)
    }

    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>, ()> {
        let mut output = vec![0u8; expected_size];
        let c = LZOContext::decompress_to_slice(&data, &mut output);

        Ok(Vec::from(c.0))
    }

}

pub struct LZOMini {}

impl Compressor for LZOMini {
    const FOURCC: &'static str = "MCOZ";

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let compressed = minilzo::compress(&data).unwrap();

        Ok(compressed)
    }

    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>, ()> {
        let decompressed = minilzo::decompress(&data, expected_size).unwrap();

        Ok(decompressed)
    }
}


pub struct LZOMini3 {}
impl Compressor for LZOMini3 {
    const FOURCC: &'static str = "MCOZ";

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let mut out = vec![0u8; data.len()];
        minilzo3::compress(&data, &mut *out, true).unwrap();

        Ok(out)
    }

    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>, ()> {
        let mut out = vec![0u8; expected_size];
        minilzo3::decompress(&data, &mut out).unwrap();

        Ok(out.to_vec())
    }
}

