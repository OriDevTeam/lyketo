// Relative Modules
mod index;
mod data;

// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use lyketo_vfs::vfs::eterfs::units::eter_unit::Unit;


pub const BGM_INDEX_PATH: &str = "tests/temp/data/eterfs_m2/encrypted_object/type_2/BGM.eix";
pub const BGM_DATA_PATH: &str = "tests/temp/data/eterfs_m2/encrypted_object/type_2/BGM.epk";


#[test]
fn load_unit() {
    let index_path = Path::new(BGM_INDEX_PATH);
    let mut unit = Unit::from_path(index_path).unwrap();

    println!("Files in unit: {:?}", unit.files_in_unit());

    println!("Files in memory (before all loaded): {:?}", unit.files_in_memory());

    unit.load_all_files();

    println!("Files in memory (after all loaded): {:?}", unit.files_in_memory());
}

