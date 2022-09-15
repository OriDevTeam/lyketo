// Standard Uses
use std::fs;

// Crate Uses
use crate::formats::map::property::Property;

// External Uses


pub fn from_file(path: &str) -> Property {
    let content = fs::read_to_string(path).expect("Cannot load chunk file");
    let property: Property = serde_json::from_str(&content).expect("Cannot lod property from content");

    property
}

