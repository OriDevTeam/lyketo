// Standard Uses

// Crate Uses

// External Uses
use lyketo_maps::{parsers::map, formats::map::Map};


const _MAPS_DIRECTORY: &str = "data/maps/";

pub fn main() {
    let _empty_map = create_empty_map();
    
    let map_legacy = load_map_legacy();
    let map_json = load_map_json();

    assert_eq!(map_legacy, map_json)
}

fn create_empty_map() -> Map {
    let (width, height) = (2, 2);
    println!("Lets make a {}x{} map to see what happens", width, height);
    
    let map: Map = Map::create(width, height);

    println!("We have a map!");

    return map;
}

fn load_map_legacy() -> Result<Map, ()> {
    let map_directory = stringify!("{}/village-legacy", MAPS_DIRECTORY);

    let map = map::legacy::parse_map_directory(map_directory);

    return map
}

fn load_map_json() -> Result<Map, ()> {
    let map_directory = stringify!("{}/village-json", MAPS_DIRECTORY);

    let map: Map = map::json::parse_map_directory(map_directory);

    return Ok(map)
}
