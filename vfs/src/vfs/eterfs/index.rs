// Standard Uses
use std::fs;
use std::collections::HashMap;

// Crate Uses

// External Uses
use eyre::{bail, Context, Result};
use glob::{glob_with, MatchOptions};
use serde::{Serialize, Deserialize};


/// Mode to search the requested resolved files in
#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
#[derive(Default)]
pub enum Search {
    #[default]
    /// Disk means that first searches are made in the physical drives/partitions
    Disk,

    /// Virtual means that first searches are made in the virtual space (RAM)
    Virtual
}


#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Index {
    pub search_mode: Search,
    pub packs: HashMap<String, Vec<String>>
}

#[allow(unused)]
impl Index {
    pub fn new() -> Self { Self { search_mode: Search::Disk, packs: Default::default() }}

    pub fn load_from_disk(path: &str) -> Result<Index> {
        todo!()
    }
}


pub fn find_index(pattern: &str) -> Result<Index> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    if let Some(entry) = glob_with(pattern, options).expect("").next() {
        let entry_path = entry?;
        let extension = entry_path.extension().unwrap().to_str().unwrap();

        let content = &fs::read_to_string(&entry_path)?;

        let index = match extension {
            "" => load_legacy(content),
            "json5" => load_json5(content),
            ext => bail!("Extension {} is not a known valid extension for Index", ext)
        }.context("TODO: panic message");

        return index
    }

    bail!("Not found")
}


pub fn load_json5(content: &str) -> Result<Index> {
    json5::from_str(content)
        .wrap_err_with(||"Couldn't parse JSON5 Index")
}


pub fn load_legacy(content: &str) -> Result<Index> {
    let mut map = Index::new();

    let lines: Vec<&str> = content.lines().collect();

    map.search_mode = match lines[0].to_lowercase().as_str() {
        "file" => Search::Disk,
        "pack" => Search::Virtual,
        _ => Search::Virtual
    };

    for i in (1..lines.len()).step_by(2) {
        let name = lines[i];
        let path = lines[i + 1].to_string();

        if !map.packs.entry(name.to_string()).or_default().contains(&path) {
            if let Some(val) = map.packs.get_mut(name) {
                val.push(path)
            };
        }
    }

    Ok(map)
}

