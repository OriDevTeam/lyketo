// Relative Modules
pub mod evfs;
pub mod eterfs;
pub mod file;
pub mod unit;
pub mod handler;
pub mod utils;

// Standard Uses

// Crate Uses

// External Uses
use anyhow::Result;


pub trait FileSystem {
    fn load_disk_dir(&mut self, path: &str) -> Result<()>;
    fn load_unit_pattern(&mut self, pattern: &str) -> Result<()>;
    fn load_unit_files(&mut self, paths: Vec<String>) -> Result<()>;
}

