// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc::FourCC;

// External Uses
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};


pub struct Header {
    pub magic: FourCC,
    pub version: u32,
    pub element_count: u32
}

impl Header {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let magic = cursor.read_u32::<LittleEndian>()?;
        let version = cursor.read_u32::<LittleEndian>()?;
        let element_count = cursor.read_u32::<LittleEndian>()?;

        Ok(Self {
            magic,
            version,
            element_count
        })
    }
}

