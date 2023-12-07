// Relative Modules
pub(crate) mod snappy;
pub(crate) mod lzo_mini;

// Standard Uses

// Crate Uses
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::Result;


pub trait Compressor {
    const FOURCC: FourCC;
    const NAME: &'static str;

    fn compress(data: &[u8]) -> Result<Vec<u8>>;
    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>>;
}


pub struct None;

impl Compressor for None {
    const FOURCC: FourCC = 0;
    const NAME: &'static str = "None";

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        // TODO; Decide what to really do here, give the same data back or panic
        Ok(data.to_vec())
    }

    #[allow(unused)]
    fn decompress(data: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        // TODO: Check note on the compress function above, same applies here
        Ok(data.to_vec())
    }
}

