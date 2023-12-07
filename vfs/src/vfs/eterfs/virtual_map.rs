// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::vfs::file::MemFile;
use crate::utils::CRC32;

// External Uses
use eyre::Result;
use crc32fast::Hasher;


#[allow(unused)]
#[derive(Default)]
pub struct VirtualMap {
    files: HashMap<CRC32, Vec<MemFile>>,
    names: HashMap<String, CRC32>
}

#[allow(unused)]
impl VirtualMap {
    pub fn has_file_by_crc(&self, hash: CRC32) -> bool {
        todo!()
    }

    pub fn get_file_by_crc(&self, hash: CRC32) -> Result<MemFile> {
        todo!()
    }

    pub fn has_file_by_name(&self, pattern: &str) -> bool {
        todo!()
    }

    pub fn get_file_by_name(&self, pattern: &str) -> Result<MemFile> {
        todo!()
    }

    pub fn add_file(&mut self, name: String, file: MemFile) {
        let mut hasher = Hasher::new();

        hasher.update(&file);
        let checksum = hasher.finalize();

        self.files.entry(checksum).or_default().push(file);
        self.names.insert(name, checksum);
    }
}

