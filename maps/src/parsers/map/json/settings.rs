use std::fs;

use crate::formats::map::settings::Settings;

pub fn from_file(path: &str) -> Settings {
    let content = fs::read_to_string(path)
    // .expect(format!("Cannot load chunk file from {}", path))
    // .as_str();
    .unwrap();

    let settings: Settings = serde_json::from_str(&content).unwrap();
    
    return settings
}
