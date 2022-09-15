// Relative Modules
pub(crate) mod snappy;
pub(crate) mod lzo;

// Standard Uses

// Crate Uses

// External Uses


pub trait Compressor {
    // type Error;
    const FOURCC: &'static str;

    fn compress(data: &Vec<u8>) -> Result<Vec<u8>, ()>;
    fn decompress(data: &Vec<u8>, expected_size: usize) -> Result<Vec<u8>, ()>;
}


pub struct None;

impl Compressor for None {
    // type Error = ();
    const FOURCC: &'static str = "";

    fn compress(_data: &Vec<u8>) -> Result<Vec<u8>, ()> { unimplemented!(); }
    fn decompress(_data: &Vec<u8>, _expected_size: usize) -> Result<Vec<u8>, ()> { unimplemented!(); }
}

