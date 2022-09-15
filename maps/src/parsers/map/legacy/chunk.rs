// Relative Modules
pub mod attributes;
pub mod height;
pub mod water;

// Standard Uses
use std::{fs::File};

// Crate Uses
use crate::formats::map::chunk::Chunk;


pub fn from_legacy_file(directory: &str) -> Chunk {
    let file = File::options()
    .read(true)
    .open(directory)
    .expect(stringify!("Cannot load chunk file from {}", directory));
    
    let chunk = from_legacy_bytes(file);
    return chunk
}

pub fn from_legacy_bytes(_content: File) -> Chunk {
    let _chunk = Chunk::new();

    return _chunk
}
