// Standard Uses
use std::io::Read;

// Crate Uses
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::Result;
use byteorder::{LittleEndian, ReadBytesExt};


pub struct Header {
    pub magic: FourCC,
    pub header_size: u32,
    pub stride: u32,
    pub element_count: u32,
    pub expected_content_size: u32,
}

impl Header {
    pub fn from_bytes<R: Read>(read: &mut R) -> Result<Self> {
        // let content_size = (data.len() - 4 - 4 - 4) as u32;

        let magic: FourCC = read.read_u32::<LittleEndian>()?;

        let version = read.read_u32::<LittleEndian>()?;
        let stride = read.read_u32::<LittleEndian>()?;

        // TODO: Move this after the stride check
        let element_count = read.read_u32::<LittleEndian>()?;
        let expected_content_size = read.read_u32::<LittleEndian>()?;

        Ok(Self {
            magic, header_size: version, stride,
            element_count, expected_content_size
        })
    }
}

