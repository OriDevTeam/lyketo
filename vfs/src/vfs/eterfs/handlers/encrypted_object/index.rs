// Relative Modules
pub mod file;

// Standard Uses

// Crate Uses
use crate::formats::encrypted_object::header::Header;
use crate::vfs::eterfs::handlers::encrypted_object::index::file::File;

// External Uses
use anyhow::Result;


const FILE_NAME_LEN: usize = 160;


pub struct Index {
    pub header: Header,
    pub files: Vec<Box<dyn File>>
}


#[allow(unused)]
impl Index {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        todo!()
    }
}

