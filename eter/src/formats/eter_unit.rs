use std::mem;
use crate::formats::eter_unit::header::Header;

// Relative Modules
mod header;
mod file;

// Standard Uses

// Crate Uses
use crate::formats::eter_unit::file::File;

// External Uses
use anyhow::{bail, Result};


pub struct Unit {
    pub files: Vec<File>
}

impl Unit {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let header = Header::from_bytes(data[..mem::size_of::<Header>()].to_vec())?;
        let elements_data = &data[mem::size_of::<Header>()..];

        let expected_size = header.element_count as usize * mem::size_of::<File>();

        if expected_size != elements_data.len() {
            bail!("Expected element size of {}, got {} instead",
                header.element_count as usize, elements_data.len()
            )
        }

        let mut files = vec![];

        for index in 0..header.element_count as usize {
            files.push(File::from_bytes(
                elements_data[index..index + mem::size_of::<File>()].to_vec()
            )?);
        }

        Ok(Self { files })
    }
}

