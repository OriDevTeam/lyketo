// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc;
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;


lazy_static!(
    static ref INDEX_CC: FourCC = four_cc::from_chars(['E', 'P', 'K', 'D']);
    static ref DATA_CC: FourCC = *INDEX_CC;
);

#[allow(unused)]
pub enum Version {
    V2 = 2
}


#[allow(unused)]
pub struct Header {
    pub magic: FourCC,
    pub version: u32,
    pub element_count: u32
}

#[allow(unused)]
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

