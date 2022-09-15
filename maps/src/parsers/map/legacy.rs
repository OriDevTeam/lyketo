// Relative Modules
pub mod property;
pub mod settings;
pub mod chunk;
pub mod segment;

// Standard Uses
use std::vec;

// Crate Uses
use crate::{
    formats::map::{chunk::Chunk, Map},
    parsers::map::legacy::chunk::from_legacy_file
};


pub fn parse_map_directory(directory: &str) -> Result<Map, ()>{
    println!("Lets parse {}", directory);

    let settings = settings::from_file(stringify!("{}/setting.txt", directory));
    let property = property::from_file(stringify!("{}/mapproperty.txt", directory));

    let chunk_directories: Vec<String> = vec![];
    let mut chunks: Vec<Chunk> = vec![];
    
    for chunk_directory in chunk_directories {
        let chunk = from_legacy_file(&chunk_directory);
        chunks.push(chunk);
    }


    let map: Map = Map {
        settings,
        property,
        chunks,
        chunk_data: vec!(),
    };

    return Ok(map)
}

