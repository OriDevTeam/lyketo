// Relative Modules
pub mod attributes;
pub mod water;
pub mod height;


// Standard Uses

// Crate Uses
use super::{MAX_CHUNKS, ACTUAL_TILES_PER_CHUNK, HEIGHTS_PER_CHUNK};
use self::{
    attributes::Attributes,
    water::Water, height::Height
};


// External Uses
use nalgebra_glm::{U8Vec2};


#[derive(Debug, PartialEq)]
pub struct ChunkData {
    pub attributes: Attributes,
    pub water: Water,
    pub height: Height
    // pub tiles: Vec<Tile>
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub rectangle: U8Vec2,
    pub heights: Vec<u16>,
    pub tiles: Vec<u16>,
    pub texture_set: u16
}

impl Chunk {
    pub(crate) fn new() -> Self { todo!() }
}

impl Chunk {

    pub fn generate () -> Chunk {
        let chunk = Chunk {
            rectangle: U8Vec2::from([MAX_CHUNKS, MAX_CHUNKS]),
            heights: Vec::new(),
            tiles: Vec::new(),
            texture_set: 0,
        };

        return chunk;
    }

    pub fn id(self) -> u32 {
        return (self.rectangle.x * self.rectangle.y).into()
    }

    pub fn size(&self) -> u16 {
        return (self.rectangle.x * self.rectangle.y).into()
    }

    pub fn height_at(&self, x: u16, y: u16) -> u16 {
        if x > HEIGHTS_PER_CHUNK || y > HEIGHTS_PER_CHUNK {
            return 0
        }

        let index: usize = (x * y).try_into().unwrap();
        return self.heights[index]
    }

    pub fn tile_at(&self, x: u16, y:u16) -> u16 {
        if x > ACTUAL_TILES_PER_CHUNK || y > ACTUAL_TILES_PER_CHUNK {
            return 0;
        }

        // let pathm: f32 = (x + y * 131).into() * 0.5 / 200.0;
        //return self.tiles[pathm]
        return 0
    }

    pub fn texture_set(&self) -> u16{
        return self.texture_set
    }
}

