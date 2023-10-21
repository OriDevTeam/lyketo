// Relative Modules
pub mod header;
pub mod mmpt;

// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc;
use crate::utils::key::Key;
use crate::utils::four_cc::FourCC;
use crate::formats::encrypted_object::types::Type6;
use crate::formats::protos::mob_proto::mmpt::MAGIC_VERSION_2_FCC;

// External Uses
use anyhow::{Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;
use crate::formats::{MCSP_FOURCC, MMPT_FOURCC};


lazy_static!(
    static ref DEFAULT_KEY: Key = Key::from_segments_u32(
        vec![4813894, 18955, 552631, 6822045]
    );
);


pub const CHARACTER_NAME_LENGTH: usize = 24;


pub struct MobProto {}

impl MobProto {

    #[allow(unused)]
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let content_size = (data.len() - 4 - 4 - 4) as u32;

        let mut cursor = Cursor::new(data);

        let magic: FourCC = cursor.read_u32::<LittleEndian>()?;

        if magic != *MAGIC_VERSION_2_FCC {
            bail!("Expected magic v2, got {} instead", magic)
        }

        let element_count = cursor.read_u32::<LittleEndian>()?;

        let expected_content_size = cursor.read_u32::<LittleEndian>()?;

        if content_size != expected_content_size {
            bail!("Expected {expected_content_size} after metadata, got {content_size} instead", )
        }
        /*
        let mut table_data: [u8; mem::size_of::<Mobs>()] = [];
        cursor.read(&mut table_data)?;

        let table = mmpt::Table::from_bytes(table_data.to_vec())?;
        */

        // let table = mmpt::Table::from_cursor(&mut cursor);

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

