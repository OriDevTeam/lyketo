// Relative Modules
use std::fs;
use std::path::Path;

// Standard Uses

// Crate Uses
use eyre::{bail, Context, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Settings {
    // r#type: String = "MapSetting
    cell_scale: f64,
    height_scale: f64,
    view_radius: f64,
    map_size: [u8; 2],
    base_position: [f64; 2],
    texture_set: String,
    environment: String
}

impl Settings {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(&path)?;
        let extension = path.extension().unwrap().to_str().unwrap();

        let settings = match extension {
            "json" => from_json(contents),
            "json5" => from_json5(contents),
            "txt" => from_m2s(contents),
            _ => { bail!("No support for extension {extension}") }
        };

        settings
    }
}


pub fn from_json(contents: String) -> Result<Settings> {
    serde_json::from_str::<Settings>(&*contents).context("Could not load JSON format")
}

pub fn from_json5(contents: String) -> Result<Settings> {
    json5::from_str::<Settings>(&*contents).context("Could not load JSON5 format")
}

pub fn from_m2s(_contents: String) -> Result<Settings> {
    todo!()
}


