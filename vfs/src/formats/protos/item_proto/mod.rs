// Relative Modules
pub mod header;
pub mod stride_156;
pub mod stride_171;
pub mod stride_199;

// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc;
use crate::utils::key::Key;
use crate::utils::four_cc::FourCC;
use crate::formats::encrypted_object::types::Type6;

// External Uses
use anyhow::{Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;
use crate::formats::{MCSP_FOURCC, MIPX_FOURCC, MMPT_FOURCC};


lazy_static!(
    static ref DEFAULT_KEY: Key = Key::from_segments_u32(
        vec![173217, 72619434, 408587239, 27973291]
    );
);


pub const CHARACTER_NAME_LENGTH: usize = 24;


pub struct ItemProto {}

impl ItemProto {

    #[allow(unused)]
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let content_size = (data.len() - 4 - 4 - 4) as u32;

        let mut cursor = Cursor::new(data);

        let magic: FourCC = cursor.read_u32::<LittleEndian>()?;

        if magic != *MIPX_FOURCC {
            bail!("Expected magic v2, got {} instead", magic)
        }

        let version = cursor.read_u32::<LittleEndian>()?;
        let stride = cursor.read_u32::<LittleEndian>()?;

        // TODO: Move this after the stride check
        let element_count = cursor.read_u32::<LittleEndian>()?;
        let expected_content_size = cursor.read_u32::<LittleEndian>()?;


        let expected_stride = std::mem::size_of::<stride_199::ItemTable>();
        let item_table_align = std::mem::align_of::<stride_199::ItemTable>();
        if stride != expected_stride as u32 {
            bail!(
                "Expected Stride(Table Size) to be {} but got {} instead",
                expected_stride, stride
            )
        }

        println!("do");
        todo!()
    }

    #[allow(unused)]
    pub fn from_file(data: Vec<u8>) -> Result<Self> {
        let content_size = (data.len() - 4 - 4 - 4) as u32;

        let mut cursor = Cursor::new(data);

        let magic: FourCC = cursor.read_u32::<LittleEndian>()?;

        if magic != *MMPT_FOURCC {
            bail!(
                "Expected Magic/FourCC {}({}), but got {}({}) instead",
                *MMPT_FOURCC, four_cc::to_string(&MMPT_FOURCC),
                magic, four_cc::to_string(&magic)
            )
        }

        let element_count = cursor.read_u32::<LittleEndian>()?;

        let expected_content_size = cursor.read_u32::<LittleEndian>()?;

        if content_size != expected_content_size {
            bail!("Expected {expected_content_size} after metadata, got {content_size} instead", )
        }

        let handler = Type6::with_properties(
            DEFAULT_KEY.clone(),
            MCSP_FOURCC.clone(), MCSP_FOURCC.clone()
        );
        let object = cursor.into_inner()[4 + 4 + 4..content_size as usize + 8].to_vec();
        let data = handler.deserialize(object, true).unwrap();


        todo!()
    }
}

