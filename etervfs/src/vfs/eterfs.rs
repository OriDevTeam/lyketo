pub mod index;
pub mod vmap;
pub mod dmap;

// Standard Uses

use std::rc::Rc;
// Crate Uses
use crate::vfs::FileSystem;
use crate::vfs::eterfs::dmap::DriveMap;
use crate::vfs::eterfs::vmap::VirtualMap;
use crate::vfs::eterfs::index::find_index;
use crate::vfs::handler::Handler;
use crate::vfs::file::MemFile;

// External Uses
use anyhow::{Context, Result};


pub type CRC32 = u32;


#[allow(unused)]
pub struct Eter {
    pub drive_map: DriveMap,
    pub virtual_map: VirtualMap,
    pub handlers: Vec<Rc<Box<dyn Handler>>>
}


impl Eter {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            drive_map: DriveMap::new(),
            virtual_map: VirtualMap::new(),
            handlers: vec![]
        }
    }

}


#[allow(unused)]
impl Eter {

    /// Adds and registers a handler and gives a reference for it back
    pub fn add_handler(&mut self, handler: Rc<Box<dyn Handler>>) {
        self.handlers.push(handler);
    }

    pub fn add_handler_inv(&mut self, handler: Box<Rc<dyn Handler>>) {
        todo!()
    }

    pub fn get_file(&self, pattern: &str) -> Result<MemFile> {
        todo!()
    }
}



impl FileSystem for Eter {

    #[allow(unused)]
    fn load_disk_dir(&mut self, path: &str) -> Result<()> {
        let pattern = format!("{}index*", path);

        let index = find_index(&pattern)
            .context(format!("No Index registry found in {}", path));

        self.drive_map.register_index(&index.unwrap());

        Ok(())
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

