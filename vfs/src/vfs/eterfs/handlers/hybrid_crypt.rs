// Standard Uses

// Crate Uses
use crate::vfs::FileSystem;
use crate::vfs::handler::Handler;
use crate::vfs::unit::Unit;

// External Uses
use anyhow::{bail, Result};


pub struct HybridCrypt {
    vfs: Option<Box<dyn FileSystem>>
}


#[allow(unused)]
impl HybridCrypt {
    pub fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()>{
        if !self.vfs.is_some() {
            bail!(
                r#"No virtual file system is set, the handler cannot put virtual files anywhere.
                Register this handler in a VFS to handle with units and virtual files.
                "#
            )
        }

        Ok(())
    }

    pub fn refresh_unit(&mut self) {

    }
}


impl HybridCrypt {

}

#[allow(unused)]
impl Handler for HybridCrypt {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()> {
        todo!()
    }
}


#[allow(unused)]
enum Cipher {
    Cammelia,
    Twofish,
    XTEA
}