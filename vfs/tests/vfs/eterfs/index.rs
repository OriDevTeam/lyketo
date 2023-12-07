// Standard Uses
use std::{env, fs};
use eyre::WrapErr;

// Crate Uses

// External Uses
use lyketo_vfs::vfs::eterfs::index;
use lyketo_vfs::vfs::eterfs::index::Search;


pub const LEGACY_INDEX_PATH: &str = "tests/__data__/etervfs_m2/Index";
pub const JSON5_INDEX_PATH: &str = "tests/__data__/etervfs_m2/Index.json5";


#[test]
pub fn load_legacy() {
    println!("{:?}", env::current_dir());

    let content = fs::read_to_string(LEGACY_INDEX_PATH)
        .wrap_err_with(|| format!("Issue with file at path '{LEGACY_INDEX_PATH}':"))
        .unwrap();

    let legacy = index::load_legacy(content.as_str()).unwrap();

    assert_eq!(legacy.search_mode, Search::Virtual);
}

#[test]
pub fn load_json5() {
    println!("{:?}", env::current_dir());

    let content = fs::read_to_string(JSON5_INDEX_PATH)
        .wrap_err_with(|| format!("Issue with file at path '{JSON5_INDEX_PATH}':"))
        .unwrap();

    let json5 = index::load_json5(content.as_str()).unwrap();

    assert_eq!(json5.search_mode, Search::Disk);
}

