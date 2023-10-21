// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;

// External Uses
use anyhow::Result;
use rust_lzo::LZOContext;


#[derive(Default)]
pub struct LZO {}

impl Compressor for LZO {
    // const FOURCC: u32 = MAGIC_MCOZ;
    const NAME: &'static str = "LZO";

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let compressed = minilzo3::compress_vec(&data, true).unwrap();

        Ok(compressed)
    }

    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        let mut output = vec![0u8; expected_size];
        let c = LZOContext::decompress_to_slice(&data, &mut output);

        Ok(Vec::from(c.0))
    }

}
