// Standard Uses

// Crate Uses
use crate::vfs::handler::Handler;
use crate::vfs::FileSystem;
use crate::vfs::unit::Unit;

// External Uses
use anyhow::{Result};


#[allow(unused)]
pub struct Panama {
    vfs: Box<dyn FileSystem>,
    key: Option<[u32; 4]>
}


impl Panama {
    pub fn new(vfs: Box<dyn FileSystem>) -> Self { Self { vfs, key: None }}

    pub fn set_key(&mut self, key: [u32; 4]) {
        self.key = Option::from(key)
    }
}


#[allow(unused)]
impl Handler for Panama {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()> {
        todo!()
    }
}