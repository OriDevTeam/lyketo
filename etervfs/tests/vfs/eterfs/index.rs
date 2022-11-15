// Standard Uses
use std::{env, fs};

// Crate Uses

// External Uses
use etervfs::vfs::eterfs::index;
use etervfs::vfs::eterfs::index::Search;


#[test]
pub fn load_legacy() {
    println!("{:?}", env::current_dir());

    let content = fs::read_to_string("tests/data/eterfs_m2/Index").unwrap();
    let legacy = index::load_legacy(content.as_str()).unwrap();

    assert_eq!(legacy.search_mode, Search::Virtual);
}

#[test]
pub fn load_json5() {
    let content = fs::read_to_string("tests/data/eterfs_m2/Index.json5").unwrap();
    let json5 = index::load_json5(content.as_str()).unwrap();

    assert_eq!(json5.search_mode, Search::Disk);
}

