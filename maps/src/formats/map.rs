// Relative Modules
pub mod settings;
pub mod property;
pub mod chunk;
pub mod segment;
pub mod mesh;
pub mod texture;
pub mod vertex;


// Crate Uses
use super:: {
    map::chunk::{Chunk, ChunkData},
    map::property::{Property, MapType},
    map::settings::Settings,
};


// TODO; Perhaps more than 255 chunks or segments might be a lot, check what it impacts
pub const MAX_CHUNKS: u8 = 138;
pub const MAX_SEGMENTS: u8 = 128;

pub const SEGMENTS_PER_CHUNK: u8 = 16;
pub const HEIGHTS_PER_CHUNK: u16 = 131;
pub const TILES_PER_CHUNK: u16 = 128;
pub const ACTUAL_TILES_PER_CHUNK: u16 = 258;

pub const HEIGHT_MULTIPLIER: f32 = 0.5;


#[derive(Debug, PartialEq)]
pub struct Map {
    pub property: Property,
    pub settings: Settings,
    pub chunks: Vec<Chunk>,
    pub chunk_data: Vec<ChunkData>
}

impl Map {

    /// Generate a map with given width and height)
    pub fn create (_width: i32, _height: i32) -> Map {
        let mut map = Map {
            property: Property { map_type: MapType::Outdoor },
            settings: Settings::new(),
            chunks: vec!(),
            chunk_data: vec!(),
        };

        // TODO: The traits that use the Map structure should be the ones to decide the limits instead
        //       So the limits should be moved elsewhere, and passed from somewhere to here
        for _x in 0.._width {
            map.chunks.push(Chunk::generate());

            for _y in 0.._height - 1 {
                map.chunks.push(Chunk::generate());
            }
        }

        return map
    }
}
