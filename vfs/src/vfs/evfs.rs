// Relative Modules
pub mod path;

// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::vfs::FileSystem;
use crate::vfs::file::File;

// External Uses
use anyhow::Result;


#[allow(unused)]
pub struct EvFS {
    index: HashMap<String, Box<dyn File>>
}


impl FileSystem for EvFS {

    #[allow(unused)]
    fn load_disk_dir(&mut self, path: &str) -> Result<()> {
        todo!()
    }

    #[allow(unused)]
    fn load_unit_pattern(&mut self, pattern: &str) -> Result<()> {
        todo!()
    }

    #[allow(unused)]
    fn load_unit_files(&mut self, paths: Vec<String>) -> Result<()> {
        todo!()
    }
}

