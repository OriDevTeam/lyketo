// Relative Modules

// Standard Uses

// Crate Uses

// External Uses
use lyketo_vfs::formats::encrypted_object::types::Type2;
use lyketo_vfs::formats::encrypted_object::types::type_2::ETER_INDEX_KEY;
use lyketo_vfs::formats::{MCOZ_FOURCC, MCSP_FOURCC};


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
    println!("\nCreating and serializing a hello message:");

    let packer = Type2::with_properties(
        ETER_INDEX_KEY.clone(), *MCOZ_FOURCC, *MCOZ_FOURCC
    );

    let serialized = packer.serialize(RAW_DATA.to_vec()).unwrap();

    assert_eq!(serialized, EXPECTED_SERIALIZED_DATA);
}


#[test]
pub fn create_deserialize_object() {
    println!("\nDeserializing a hello message:");

    // let packer = Type2::with_key(ETER_INDEX_KEY.clone());
    let packer = Type2::with_properties(
        ETER_INDEX_KEY.clone(), *MCOZ_FOURCC, *MCSP_FOURCC
    );

    let deserialized = packer.deserialize(EXPECTED_SERIALIZED_DATA.to_vec(), true).unwrap();

    assert_eq!(deserialized, RAW_DATA);
}

