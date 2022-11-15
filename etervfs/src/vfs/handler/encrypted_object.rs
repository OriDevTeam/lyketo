// Relative Modules
pub mod pack;
pub mod index;

// Standard Uses

// Crate Uses
use crate::vfs::handler::Handler;
use crate::vfs::handler::encrypted_object::key::Key;
use crate::vfs::FileSystem;
use crate::vfs::unit::Unit;


// External Uses
use anyhow::{Result};


#[allow(unused)]
pub struct EncryptedObject {
    vfs: Box<dyn FileSystem>,
    pub compression_key: Option<Key>,
}

impl EncryptedObject {
    pub fn new(vfs: Box<dyn FileSystem>) -> Self { Self { vfs, compression_key: Key::new() }}
}


#[allow(unused)]
impl Handler for EncryptedObject {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()> {
        todo!()
    }
}

