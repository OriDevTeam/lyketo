// Standard Uses
use std::path::Path;

// Crate Uses


// External Uses
use lyketo_vfs::formats::encrypted_object::methods::basic::types::Type6;
use lyketo_vfs::formats::encrypted_object::methods::basic::types::type_2::ETER_INDEX_KEY;
use lyketo_vfs::vfs::eterfs::units::eter_unit::Unit;


pub const LOCALE_INDEX_PATH: &str = "tests/__temp_data__/eterfs_m2/encrypted_object/type_6/locale_en.eix";
pub const LOCALE_DATA_PATH: &str = "tests/__temp_data__/eterfs_m2/encrypted_object/type_6/locale_en.epk";


#[test]
pub fn load_index_deserialize() {
    let path = Path::new(LOCALE_INDEX_PATH);
    let file = std::fs::read(path).unwrap();

    // let packer = Type2::with_key(ETER_INDEX_KEY.clone());
    let packer = Type6::with_key(ETER_INDEX_KEY.clone());

    let index_object = packer.deserialize(file, true).unwrap();

    println!("Index Object ({} bytes): ", index_object.len());
    println!("{}", String::from_utf8_lossy(&index_object));

    // assert_eq!(EXPECTED_DECOMPRESSED_INDEX, index_object.as_slice());
}


#[test]
fn load_unit() {
    let index_path = Path::new(LOCALE_INDEX_PATH);
    let mut unit = Unit::from_path(index_path, true).unwrap();

    println!("Files in unit: {:?}", unit.files_in_unit());

    println!("Files in memory (before all loaded): {:?}", unit.files_in_memory());

    unit.load_all_files();

    println!("Files in memory (after all loaded): {:?}", unit.files_in_memory());
}

