// Standard Uses

// Crate Uses
use crate::utils::four_cc::FourCC;

// External Uses


pub struct Header {
    pub magic: FourCC,
    pub header_size: u32,

}

