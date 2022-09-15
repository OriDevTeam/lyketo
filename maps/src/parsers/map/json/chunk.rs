
// Crate Uses
use crate::{formats::map::chunk::{
    ChunkData}, 
    parsers::map
};


pub fn from_directory(directory: &str) -> ChunkData {
    let attributes_path = format!("{}/attr.atr", directory);
    let height_path = format!("{}/height.raw", directory);
    let water_path = format!("{}/water.wtr", directory);
    // let shadows_path = format!("{}/shadowmap.raw", directory);
    // let tiles_path = format!("{}/tile.raw", directory);

    let chunk = ChunkData {
        attributes: map::legacy::chunk::attributes::from_file(&attributes_path),
        height: map::legacy::chunk::height::from_file(&height_path),
        water: map::legacy::chunk::water::from_file(&water_path),
    };

    return chunk
}
