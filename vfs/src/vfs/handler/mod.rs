// Relative Modules
pub mod mock;

// Standard Uses

// Crate Uses
use crate::vfs::unit::Unit;

// External Uses
use eyre::Result;


pub trait Handler {
    fn add_unit(&mut self, unit: Box<dyn Unit>) -> Result<()>;
}

