// Standard Uses

// Crate Uses
use crate::vfs::file::MemFile;

// External Uses
use eyre::Result;


pub struct Data {
    pub files: Vec<MemFile>
}

#[allow(unused)]
impl Data {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        todo!()
    }
}

