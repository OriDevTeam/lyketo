// Standard Uses

// Crate Uses

// External Uses


const MAPS_DIRECTORY: &str = "data/maps/";

#[test]
fn load_legacy_map_directory() {
    let _map_directory = format!("{}/test_map_legacy/", MAPS_DIRECTORY);

    // legacy::map::parse_map_directory(&map_directory);
}

#[test]
fn load_json_map_directory() {
    let _map_directory = format!("{}/test_map_json/", MAPS_DIRECTORY);
    // let _map = json::parse_map_directory(&map_directory);
}
