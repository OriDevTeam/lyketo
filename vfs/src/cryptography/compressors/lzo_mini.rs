// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;
use crate::formats::MCOZ_FOURCC;
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::{Result, WrapErr};
use minilzo;


pub struct LZOMini {}

impl Compressor for LZOMini {
    const FOURCC: FourCC = *MCOZ_FOURCC;
    const NAME: &'static str = "LZOMini";

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let compressed = minilzo::compress(data)
            .wrap_err_with(|| "Couldn't compress with LZOMini")?;

        Ok(compressed)
    }

    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        let decompressed = minilzo::decompress(data, expected_size)
            .wrap_err_with(|| "Couldn't decompress with LZOMini")?;

        Ok(decompressed)
    }
}
