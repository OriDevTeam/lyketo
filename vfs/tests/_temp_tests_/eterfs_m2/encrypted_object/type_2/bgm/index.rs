// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses
use crate::_temp_tests_::eterfs_m2::encrypted_object::type_2::bgm::BGM_INDEX_PATH;

// External Uses
use lyketo_vfs::formats::encrypted_object::types::type_2::ETER_INDEX_KEY;
use lyketo_vfs::formats::encrypted_object::types::Type2;
use lyketo_vfs::formats::MCOZ_FOURCC;


const EXPECTED_DECOMPRESSED_INDEX: [u8; 1356] = [
    69, 80, 75, 68, 2, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 98, 103, 109, 47, 99, 104, 97, 114, 97, 99,
    116, 101, 114, 115, 101, 108, 101, 99, 116, 46, 109, 112, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 141, 134, 187, 166, 64, 87, 44, 0, 0, 88, 44, 0, 3, 165, 163, 158, 0, 0, 0, 0, 1, 254,
    100, 7, 1, 0, 0, 0, 98, 103, 109, 47, 100, 101, 115, 101, 114, 116, 46, 109, 112, 51, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 31, 13, 131, 5, 4, 57, 0, 0, 5, 57,
    0, 160, 48, 238, 239, 0, 88, 44, 0, 1, 254, 18, 5, 2, 0, 0, 0, 98, 103, 109, 47, 108, 111, 103,
    105, 110, 95, 119, 105, 110, 100, 111, 119, 46, 109, 112, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 202, 219, 250, 137, 114, 40, 7, 0, 0, 41, 7, 0, 185, 213, 31, 147, 0, 93, 101,
    0, 1, 254, 18, 5, 3, 0, 0, 0, 98, 103, 109, 47, 109, 50, 98, 103, 46, 109, 112, 51, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 233, 51, 53, 159, 90, 57, 0,
    0, 91, 57, 0, 66, 103, 70, 56, 0, 134, 108, 0, 1, 254, 240, 9, 4, 0, 0, 0, 98, 103, 109, 47,
    119, 101, 100, 100, 105, 110, 103, 46, 109, 112, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 167, 142, 55, 213, 199, 146, 14, 0, 0, 147, 14, 0, 233, 125, 123, 23, 0,
    225, 165, 0, 1, 254, 18, 5, 5, 0, 0, 0, 98, 103, 109, 47, 121, 109, 105, 114, 95, 99, 104, 97,
    114, 97, 99, 116, 101, 114, 115, 101, 108, 101, 99, 116, 46, 109, 112, 51, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 254, 216, 77, 192, 98, 83, 7, 0, 0, 84, 7, 0, 34, 38, 96, 13, 0, 116, 180, 0, 1, 254, 18, 5,
    6, 0, 0, 0, 98, 103, 109, 47, 121, 109, 105, 114, 95, 108, 111, 103, 105, 110, 95, 119, 105,
    110, 100, 111, 119, 46, 109, 112, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 219, 185, 68, 19, 29,
    174, 4, 0, 0, 175, 4, 0, 129, 127, 8, 251, 0, 200, 187, 0, 1, 254, 18, 5
];


#[test]
pub fn load_index_deserialize() {
    let path = Path::new(BGM_INDEX_PATH);
    let file = fs::read(path).unwrap();

    // let packer = Type2::with_key(ETER_INDEX_KEY.clone());
    let packer = Type2::with_properties(
        ETER_INDEX_KEY.clone(), *MCOZ_FOURCC, *MCOZ_FOURCC
    );

    let index_object = packer.deserialize(file, true).unwrap();

    println!("Index Object ({} bytes): ", index_object.len());
    println!("{}", String::from_utf8_lossy(&index_object));

    assert_eq!(EXPECTED_DECOMPRESSED_INDEX, index_object.as_slice());
}
