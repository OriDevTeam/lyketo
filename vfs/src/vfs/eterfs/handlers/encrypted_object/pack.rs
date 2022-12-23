// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses
use crate::vfs::unit::Unit;
use crate::vfs::file::MemFile;

// External Uses
use anyhow::Result;


#[allow(unused)]
pub struct Pack {}


#[allow(unused)]
impl Pack {
    pub fn load_from_disk(path: &str) -> Result<Self> {
        let index_path = &format!("{}.eix", path);
        let index_path = Path::new(index_path);
        let data_path = Path::new(&format!("{}.eix", path));

        let index_file = fs::read(index_path)?;

        todo!()
    }
}


impl Unit for Pack {

    #[allow(unused)]
    fn has_file(&self, pattern: &str) -> bool {
        todo!()
    }

    #[allow(unused)]
    fn get_file(&self, pattern: &str) -> Result<MemFile> {
        todo!()
    }

    #[allow(unused)]
    fn get_files(&self, pattern: &str) -> Result<Vec<MemFile>> {
        todo!()
    }
}

