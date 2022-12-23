// Relative Modules
pub mod crc;
pub mod md5;

// Standard Uses

// Crate Uses

// External Uses


pub trait File {}


#[allow(unused)]
pub fn load_files_information(information: Vec<u8>, count: usize) -> Vec<Box<dyn File>> {
    let files: Vec<Box<dyn File>> = Vec::with_capacity(count);

    files
}

