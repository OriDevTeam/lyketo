// Relative Modules
pub mod file;
pub mod header;

// Standard Uses
use std::mem;

// Crate Uses
use crate::vfs::eterfs::units::eter_unit::index::file::File;
use crate::vfs::eterfs::units::eter_unit::index::header::Header;

// External Uses
use eyre::{bail, Result};


#[allow(unused)]
pub struct Index {
    pub files: Vec<File>
}

#[allow(unused)]
impl Index {
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
            let start = index * mem::size_of::<File>();
            let end = start + mem::size_of::<File>();

            files.push(File::from_bytes(
                elements_data[start..end].to_vec()
            )?);
        }

        Ok(Self { files })
    }
}

