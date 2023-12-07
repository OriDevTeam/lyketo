// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;
use crate::utils::four_cc::FourCC;
use crate::formats::MCSP_FOURCC;

// External Uses
use eyre::{Result, WrapErr};
use snap::raw::{Decoder, Encoder};


#[derive(Default)]
pub struct Snappy {}


#[allow(unused)]
impl Compressor for Snappy {
    const FOURCC: FourCC = *MCSP_FOURCC;
    const NAME: &'static str = "Snappy";

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let mut compressed = Encoder::new().compress_vec(data)
            .wrap_err_with(|| "Couldn't compress with Snappy:")?;

        Ok(compressed)
    }

    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        let mut decompressed = Decoder::new().decompress_vec(data)
            .wrap_err_with(|| "Couldn't decompress with Snappy:")?;

        Ok(decompressed)
    }
}
