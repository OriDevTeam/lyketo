// Relative Modules
pub mod mock;
pub mod encrypted_object;
pub mod panama;
pub mod hybrid_crypt;
pub mod hybrid_crypt_sdb;


// Standard Uses

// Crate Uses
use crate::vfs::unit::Unit;

// External Uses
use anyhow::Result;


pub trait Handler {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()>;
}

