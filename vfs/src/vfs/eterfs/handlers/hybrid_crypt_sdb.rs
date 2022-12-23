// Standard Uses

// Crate USes
use crate::vfs::handler::Handler;
use crate::vfs::FileSystem;
use crate::vfs::unit::Unit;

// External Uses
use anyhow::{Result};


/// Hybrid Crypt Supplementary Data Block
/// This strategy is equal to regular Hybrid Crypt but the difference
/// is before the unit needs to be decompressed/decrypted the SDB has to be
/// given to restore it to a valid state (the post encrypted state)
///
/// When a unit is processed, some bytes at the end are cut out hence the name
/// Supplementary Data Block (SDB for short)
#[allow(unused)]
pub struct HybridCryptSDB {
    vfs: Box<dyn FileSystem>
}


impl HybridCryptSDB {
    pub fn new(vfs: Box<dyn FileSystem>) -> Self { Self { vfs } }
}


#[allow(unused)]
impl Handler for HybridCryptSDB {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()> {

        Ok(())
    }
}

