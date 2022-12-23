// Standard Uses

// Crate Uses
use crate::vfs::file::MemFile;

// External Uses
use anyhow::Result;


pub trait Unit {
    fn has_file(&self, pattern: &str) -> bool;
    fn get_file(&self, pattern: &str) -> Result<MemFile>;
    fn get_files(&self, pattern: &str) -> Result<Vec<MemFile>>;
}

