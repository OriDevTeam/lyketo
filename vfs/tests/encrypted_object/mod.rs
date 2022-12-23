// Relative Modules
mod type_1;
pub(crate) mod type_2;
mod panama;


// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use lyketo_vfs::formats::encrypted_object::types::TypeRaw;
use lyketo_vfs::utils::key::Key;


const MOCK_ENCRYPTED_OBJECT: &str = "eter/tests/data/mock_encrypted_object.epk";


#[test]
fn create_encrypted_object() {
    /*
    let encrypted_object = EncryptedObject::new();
    let output = encrypted_object.serialize().unwrap();

    File::create("eter/tests/data/mock_crypted_object.epk")
        .expect("Unable to create file")
        .write_all(&output[1..])
        .expect("Could not write to file");

    assert_eq!(&output[1..], b"MCOZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    */
}

#[test]
fn load_encrypted_object() {
    let path = Path::new(MOCK_ENCRYPTED_OBJECT);
    let file = fs::read(path).unwrap();

    let packer = TypeRaw::with_key(Key::default());

    let encrypted_object = packer.deserialize(file, true).unwrap();

    dbg!(encrypted_object);
}

