// Standard Uses
use std::array;
use std::ffi::{c_char, CStr};
use std::io::Cursor;


// Crate Uses

// External Uses
use eyre::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;


#[repr(C, align(4))]
#[derive(Debug)]
pub struct File {
    pub id: u32,
    pub name: [c_char; 160 + 1],
    pub name_crc: u32,
    pub raw_size: u32,
    pub data_size: u32,
    pub data_crc: u32,
    pub position: i32,
    pub r#type: i8,
}


impl File {
    pub fn name(&self) -> &CStr {
        CStr::from_bytes_until_nul(bytemuck::cast_slice(&self.name)).unwrap()
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let id = cursor.read_u32::<LittleEndian>()?;
        let name = array::from_fn(|_| cursor.read_i8().unwrap() as c_char);

        cursor.advance(3);

        let name_crc = cursor.read_u32::<LittleEndian>()?;
        let raw_size = cursor.read_u32::<LittleEndian>()?;
        let data_size = cursor.read_u32::<LittleEndian>()?;
        let data_crc = cursor.read_u32::<LittleEndian>()?;
        let position = cursor.read_i32::<LittleEndian>()?;
        let r#type = cursor.read_i8()?;

        cursor.advance(3);

        Ok(Self {
            id,
            name,
            name_crc,
            raw_size,
            data_size,
            data_crc,
            position,
            r#type,
        })
    }
}

