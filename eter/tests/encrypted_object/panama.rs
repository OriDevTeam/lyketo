// Relative Modules

// Standard Uses

// Crate Uses

// External Uses
use lyketo_eter::formats::panama::Panama;


const RAW_DATA: &[u8; 33] = b"Hello, this is an message object!";

const EXPECTED_SERIALIZED_DATA: [u8; 2] = [10, 10
];

const EXPECTED_FILENAME_KEY: [u8; 64] = [
    40, 151, 30, 222, 130, 188, 213, 83, 82, 65, 115, 11, 70, 13, 149, 208, 34, 79, 214, 53, 14,
    180, 15, 116, 56, 238, 108, 247, 44, 143, 219, 41, 242, 110, 150, 2, 4, 231, 93, 79, 186, 255,
    60, 233, 142, 153, 25, 225, 60, 244, 196, 26, 27, 215, 90, 99, 57, 135, 198, 226, 109, 171,
    174, 136
];

const KEY: [u32; 4] = [0, 0, 0, 0];


#[test]
fn create_serialize_panama_object() {
    let filename = "test_serialization.extension";

    let packer = Panama::with_key(KEY);

    let panama_key_and_serialized_object = packer.serialize(
        filename, RAW_DATA.to_vec()
    ).unwrap();

    assert_eq!(panama_key_and_serialized_object.0, EXPECTED_FILENAME_KEY);
    assert_eq!(panama_key_and_serialized_object.1, EXPECTED_SERIALIZED_DATA)
}

#[test]
fn crate_deserialize_panama_object() {

}

