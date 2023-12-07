// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use eyre::{bail, Context, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Regen {
    spawn_type: String,
    local_x: String,
    local_y: String,
    spawn_range_x: String,
    spawn_range_y: String,
    respawn_delay: String,
    spawn_chance: String,
    count: String,
    spawn_vnum: String
}

impl Regen {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let extension = path.extension().unwrap().to_str().unwrap();

        let regen = match extension {
            "json" => from_json(contents),
            "json5" => from_json5(contents),
            "txt" => from_m2s(contents),
            _ => { bail!("No support for extension {extension}") }
        };

        regen
    }
}

pub fn from_json(contents: String) -> Result<Regen> {
    serde_json::from_str::<Regen>(&*contents).context("Could not load JSON format")
}

pub fn from_json5(contents: String) -> Result<Regen> {
    json5::from_str::<Regen>(&*contents).context("Could not load JSON5 format")
}

pub fn from_m2s(_contents: String) -> Result<Regen> {
    todo!()
}


