// Relative Modules
pub mod header;
pub mod file;

// Standard Uses

// Crate Uses
use crate::vfs::utils::four_cc::FourCC;
use crate::vfs::utils::four_cc;
use crate::vfs::handler::encrypted_object::index::file::File;
use crate::vfs::handler::encrypted_object::index::header::{Header, VERSION};

// External Uses
use anyhow::{bail, Result};
use lazy_static::lazy_static;


const FILE_NAME_LEN: usize = 160;


lazy_static!(
    static ref INDEX_CC: FourCC = four_cc::from_chars(['E', 'P', 'K', 'D']);
    static ref DATA_CC: FourCC = *INDEX_CC;
);


pub struct Index {
    pub header: Header,
    pub files: Vec<Box<dyn File>>
}


impl Index {
    pub fn load(file: Vec<u8>) -> Result<Self> {
        if file.len() < 16 {
            bail!("Expected at least 16 bytes for the header")
        }

        let header = Header::manual_load((&file[..16]).try_into()?);

        if header.version != VERSION {
            bail!("Header is version '{}', expected '{}' instead", header.version, VERSION);
        }

        todo!()
    }
}

