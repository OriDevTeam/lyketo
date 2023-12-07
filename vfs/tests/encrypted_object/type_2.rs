// Relative Modules

// Standard Uses

// Crate Uses

// External Uses
use lyketo_vfs::formats::MCOZ_FOURCC;
use lyketo_vfs::formats::encrypted_object::methods::basic::types::Type2;
use lyketo_vfs::formats::encrypted_object::methods::basic::types::type_2::ETER_INDEX_KEY;


const RAW_DATA: &[u8; 33] = b"Hello, this is an message object!";

const EXPECTED_SERIALIZED_DATA: [u8; 64] = [
    77, 67, 79, 90, // Compression Magic
    48, 0, 0, 0,    // Ciphered Size
    37, 0, 0, 0,    // Compressed Size
    33, 0, 0, 0,    // Raw Object Size
    // Object Data
    49, 205, 15, 150, 231, 127, 142, 142, 12, 180, 138, 161, 16, 228, 201, 149, 223, 170, 74, 228,
    6, 2, 225, 230, 122, 59, 197, 114, 52, 117, 217, 110, 240, 220, 239, 28, 168, 51, 159, 168, 210,
    237, 102, 117, 29, 250, 188, 130
];




#[test]
pub fn create_serialize_object() {
    println!("\nCreating and serializing a hello message:");

    let packer = Type2::with_key(
        ETER_INDEX_KEY.clone(), *MCOZ_FOURCC, *MCOZ_FOURCC
    );

    let serialized = packer.serialize(RAW_DATA.to_vec(), true)
        .unwrap();

    assert_eq!(serialized, EXPECTED_SERIALIZED_DATA);
}


#[test]
pub fn create_deserialize_object() {
    println!("\nDeserializing a hello message:");

    // let packer = Type2::with_key(ETER_INDEX_KEY.clone());
    let packer = Type2::with_key(
        ETER_INDEX_KEY.clone(), *MCOZ_FOURCC, *MCOZ_FOURCC
    );

    let deserialized = packer.deserialize(
        EXPECTED_SERIALIZED_DATA.to_vec(), true
    ).unwrap();

    println!("Deserialized into (bytes): {:?}", deserialized);
    println!(
        "Deserialized into (as String): {:?}",
        String::from_utf8(deserialized.clone()).unwrap()
    );

    assert_eq!(deserialized, RAW_DATA);
}

