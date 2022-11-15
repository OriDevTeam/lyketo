// Standard Uses
use std::array;
use std::ffi::c_char;
use std::io::{Cursor, Read};

// Crate Uses

// External Uses
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};


// TODO: Padding variables most probably are C align to 4
// #[repr(C, align(4))]
pub struct File {
    pub id: u32,
    pub name: [c_char; 161],
    pub padding_1: [u8; 3],
    pub name_crc: u32,
    pub raw_size: u32,
    pub file_size: u32,
    pub file_crc: u32,
    pub position: u32,
    pub r#type: u8,
    pub padding_2 : [u8; 3]
}

impl File {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let id = cursor.read_u32::<LittleEndian>()?;
        
        // let mut name: [i8; 161] = [0i8; 161];
        // cursor.read_exact(&mut name)?;

        let name = array::from_fn(|_| cursor.read_i8().unwrap() as c_char);

        let mut padding_1 = [0u8; 3];
        cursor.read_exact(&mut padding_1)?;

        let name_crc = cursor.read_u32::<LittleEndian>()?;
        let raw_size = cursor.read_u32::<LittleEndian>()?;
        let file_size = cursor.read_u32::<LittleEndian>()?;
        let file_crc = cursor.read_u32::<LittleEndian>()?;
        let position = cursor.read_u32::<LittleEndian>()?;
        let r#type = cursor.read_u8()?;

        let mut padding_2 = [0u8; 3];
        cursor.read_exact(&mut padding_2)?;


        Ok(Self {
            id,
            name,
            padding_1,
            name_crc,
            raw_size,
            file_size,
            file_crc,
            position,
            r#type,
            padding_2
        })
    }
}

