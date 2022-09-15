// Standard Uses
use std::fs::File;
use std::io::{Read};

// Crate Uses

// External Uses
use lyketo_eter::formats::types::{Type2};


const RAW_DATA: &[u8; 33] = b"Hello, this is an message object!";

const EXPECTED_SERIALIZED_DATA: [u8; 56] = [
    77, 67, 79, 90, // Compression Magic
    40, 0, 0, 0,    // Ciphered Size
    37, 0, 0, 0,    // Compressed Size
    33, 0, 0, 0,    // Raw Object Size
    // Object Data
    100, 189, 163, 254, 38, 101, 135, 96, 55, 1, 165, 77, 42, 26, 230, 78, 58, 114, 252, 86, 66, 52,
    161, 216, 29, 89, 81, 87, 8, 129, 3, 108, 108, 252, 208, 229, 185, 29, 225, 242
];



#[test]
pub fn create_serialize_object() {
    let serialized = Type2::serialize(RAW_DATA.to_vec()).unwrap();

    assert_eq!(serialized, EXPECTED_SERIALIZED_DATA);
}


#[test]
pub fn create_deserialize_object() {
    let deserialized = Type2::deserialize(EXPECTED_SERIALIZED_DATA.to_vec()).unwrap();

    assert_eq!(deserialized, RAW_DATA);
}



#[test]
pub fn load_to_delete_eix() {
    let mut buffer = vec![];
    let _ = File::open("tests/data/to_delete.eix")
        .expect("{}")
        .read_to_end(&mut buffer)
        .expect("{}");

    let eix_raw_data = Type2::deserialize(buffer).unwrap();

    assert_eq!(eix_raw_data, vec![]);
}
