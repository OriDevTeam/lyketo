// Standard Uses

// Crate Uses
use crate::vfs::utils::four_cc::FourCC;
use crate::vfs::utils::four_cc;

// External Uses
// use anyhow::{bail, Result};
use bytes::{Buf, Bytes};

pub const VERSION: u32 = 2;


#[derive(Debug)]
#[repr(align(4))]
pub struct Header {
    pub magic: FourCC,
    pub version: u32,
    pub index_count: u32
}


#[allow(unused)]
impl Header {
    pub fn load(header: [u8; 16]) -> Self {
        todo!()
    }

    pub fn manual_load(header: [u8; 16]) -> Self {
        let mut buf = Bytes::copy_from_slice(&header[..]);

        let magic_bytes: [u8; 4] = buf.copy_to_bytes(4).to_vec().as_slice().try_into().unwrap();
        let magic = four_cc::to_string(four_cc::from_bytes(magic_bytes));

        println!("Magic: {}", magic);
        println!("Here: {:?}", header);
        todo!()
    }
}