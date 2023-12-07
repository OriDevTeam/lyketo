// Standard Uses
use std::path::Path;
use std::collections::HashMap;

// Crate Uses
use crate::vfs::eterfs::index::Index;
use crate::utils::CRC32;

// External Uses
use eyre::{bail, Result};


#[allow(unused)]
#[derive(Default)]
pub struct DriveMap {
    files: HashMap<CRC32, String>,
    // pub packs: Vec<Box<dyn Pack>>,
    base_path: String
}

impl DriveMap {
    // pub fn new () -> Self { Self { files: Default::default(), base_path: "".to_string() } }

    #[allow(unused)]
    pub fn register_index(&mut self, index: &Index) -> Result<()> {
        for (name, paths) in &index.packs {
            let pack_path = &format!("{}/{}", self.base_path, name);
            let pack_path = Path::new(pack_path);

            if pack_path.exists() {
                bail!("Could not load possible pack files for name at {:?}", pack_path)
            }

            let hash = crc32fast::hash(pack_path.to_str().unwrap().as_bytes());

            self.files.insert(hash, pack_path.to_str().unwrap().to_string());
        }

        Ok(())
    }
}

