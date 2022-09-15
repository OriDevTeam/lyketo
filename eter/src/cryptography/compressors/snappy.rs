// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;


// External Uses
// use snap;


#[derive(Default)]
pub struct Snappy {}

impl Compressor for Snappy {
    // type Error = Snappy::Error;
    const FOURCC: &'static str = "MCPZ";

    fn compress(_data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }

    fn decompress(_data: &Vec<u8>, _expected_size: usize) -> Result<Vec<u8>, ()> {
        todo!()
    }
}
