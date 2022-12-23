// Relative Modules
pub mod header;
pub mod mmpt;

// Standard Uses
use std::mem;
use std::io::{Cursor, Read};

// Crate Uses
use crate::utils::key::Key;
use crate::utils::four_cc::FourCC;
use crate::formats::protos::mob_proto::mmpt::MAGIC_VERSION_2_FCC;

// External Uses
use anyhow::{Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;


lazy_static!(
    static ref DEFAULT_KEY: Key = Key::from_decimals_u32(vec![4813894, 18955, 552631, 6822045]);
);


pub const CHARACTER_NAME_LENGTH: usize = 24;


pub struct Mobs {}

impl Mobs {

    #[allow(unused)]
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let magic: FourCC = cursor.read_u32::<LittleEndian>()?;

        if magic != *MAGIC_VERSION_2_FCC {
            bail!("Expected magic v2, got {} instead", magic)
        }

        let element_count = cursor.read_u32::<LittleEndian>()?;

        let data_size = cursor.read_u32::<LittleEndian>()?;

        let mut table_data: [u8; mem::size_of::<Mobs>()] = [];
        cursor.read(&mut table_data)?;

        let table = mmpt::Table::from_bytes(table_data.to_vec())?;

        todo!()
    }
}

