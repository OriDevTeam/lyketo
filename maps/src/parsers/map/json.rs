// Relative Modules
pub mod property;
pub mod settings;
pub mod chunk;
pub mod segment;

// Crate Uses
use crate::{
    formats::map::{Map, chunk::Chunk},
    parsers::map::json
};

// External Uses
use glob::{glob};


pub fn load_map(_path: &str) -> Map{
    // let file = json::from(path);

    let width = 0;
    let height = 0;

    let map: Map = Map::create(width, height);

    return map
}

pub fn parse_map_directory(directory: &str) -> Map {
    let settings_path = format!("{}/settings.json", directory);
    let property_path = format!("{}/property.json", directory);

    let settings = settings::from_file(&settings_path);
    let property = property::from_file(&property_path);

    
    let mut map = Map {
        property: property,
        settings: settings,
        chunks: vec!(),
        chunk_data: vec!()
    };

    let pattern = format!("{}/[0-9][0-9][0-9][0-9][0-9][0-9]", &directory);
    for entry in glob(&pattern).expect("Failed to read pattern") {
        let directory = entry.unwrap().display().to_string();

        let chunk_data = json::chunk::from_directory(&directory);
        let chunk = Chunk::new();

        map.chunks.push(chunk);
        map.chunk_data.push(chunk_data);
    }
    
    return map
}

