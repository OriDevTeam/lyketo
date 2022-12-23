// Standard Uses

// Crate Uses
use crate::vfs::handler::Handler;
use crate::vfs::file::MemFile;
use crate::vfs::FileSystem;
use crate::vfs::unit::Unit;

// External Uses
use anyhow::{Result};


#[allow(unused)]
pub struct Mock {
    vfs: Box<dyn FileSystem>,
    key: Option<String>
}


#[allow(unused)]
impl Mock {
    pub fn new(vfs: Box<dyn FileSystem>) -> Self { Self { key: None, vfs } }

    pub fn set_key(&mut self, key: String) -> Result<()> {
        self.key = Option::from(key);

        Ok(())
    }

    pub fn add_mock(&mut self, path: &str) -> Result<()> {
        todo!()
    }

    pub fn get_file(&self, p0: &str) -> Result<MemFile> {
        todo!()
    }
}

#[allow(unused)]
impl Handler for Mock {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()> {
        todo!()
    }
}

