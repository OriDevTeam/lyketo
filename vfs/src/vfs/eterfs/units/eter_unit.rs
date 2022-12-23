// Relative Modules
mod index;
mod data;

// Standard Uses
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

// Crate Uses
use crate::vfs::file::MemFile;
use crate::vfs::eterfs::units::eter_unit::index::Index;
use crate::formats::encrypted_object::types::Type2;
use crate::formats::encrypted_object::ETER_INDEX_KEY;

// External Uses
use anyhow::Result;


pub struct Unit {
    pub handler: Type2,
    pub index: Index,
    data: Vec<u8>,
    pub files: HashMap<u32, MemFile>
}


#[allow(unused)]
impl Unit {
    pub fn from_path(path: &Path) -> Result<Self> {
        let handler = Type2::with_key(ETER_INDEX_KEY.clone());

        // println!("Handling {}", path.display());

        let index_data = fs::read(path)?;
        let index_raw = handler.deserialize(index_data, false)?;
        let index = Index::from_bytes(index_raw)?;

        let unit_data_path = path.with_extension("epk");
        let unit_data = fs::read(unit_data_path.as_path()).unwrap();

        Ok(Self {
            handler,
            index,
            data: unit_data,
            files: Default::default()
        })
    }

    pub fn load_all_files(&mut self) {
        println!("Fetching {} Files\n", &self.index.files.len());

        for file in &self.index.files {
            let start = file.position as usize;
            let end = file.position as usize + file.raw_size as usize;

            let data = self.data[start..end].to_vec();

            /*
            let collector = tracing_subscriber::fmt()
                .with_max_level(Level::TRACE)
                .finish();

            let (unit_data) = tracing::collect::with_default(collector, || {
                let unit_data_path = path.with_extension("epk");
                let unit_data = fs::read(unit_data_path.as_path()).unwrap();

                unit_data
            });
            */

            println!("Handling File {} ({} size)", file.id, file.raw_size);
            // println!("Handling File {:#?}", file);

            let object_data = self.handler.deserialize(data, true).unwrap();

            println!("Raw Object Data Size: {:#?}", object_data.len());

            self.files.insert(file.id, object_data);
        }
    }

    pub fn files_in_memory(&self) -> Vec<&CStr> {
        let mut loaded = vec![];

        for (index, mem_file) in &self.files {
            let file = self.index.files.iter()
                .find(|&f| f.id == *index).unwrap();

            loaded.push(file.name())
        }

        loaded
    }

    pub fn files_in_unit(&self) -> Vec<&CStr> {
        let mut names = vec![];

        for file in &self.index.files {
            names.push(file.name());
        }

        names
    }

    pub fn fetch_mem_file(&self, name: &str) -> Result<MemFile> {
        todo!()
    }
}

