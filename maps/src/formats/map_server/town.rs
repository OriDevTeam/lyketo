// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use eyre::{bail, Context, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Town {
    local_x: u32,
    local_y: u32
}


impl Town {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).context("Something")?;
        let extension = path.extension().unwrap().to_str().unwrap();

        let settings = match extension {
            "json" => from_json(contents),
            "json5" => from_json5(contents),
            "txt" => from_m2s(contents),
            _ => { bail!("No format found for extension {extension}") }
        };

        settings
    }
}


pub fn from_json(contents: String) -> Result<Town> {
    serde_json::from_str::<Town>(&*contents).context("Could not load JSON format")
}

pub fn from_json5(contents: String) -> Result<Town> {
    json5::from_str::<Town>(&*contents).context("Could not load JSON5 format")
}

pub fn from_m2s(_contents: String) -> Result<Town> {
    todo!()
}


