/*
// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use lyketo_vfs::formats::encrypted_object::types::TypeRaw;
use lyketo_vfs::formats::{MCOZ_FOURCC, MCSP_FOURCC};
use lyketo_vfs::utils::key::Key;


const MOCK_ENCRYPTED_OBJECT: &str = "tests/__data__/mock_encrypted_object.epk";


#[test]
fn create_encrypted_object() {
    // TODO: I think the idea here was to replace TypeRaw with EncryptedObject because
    //       its backed by a VFS, confirm this intent later

    /*
    let eter_vfs = Box::new(Eter::new());
    let encrypted_object = EncryptedObject::new(eter_vfs);
    */

    /*
    let encrypted_object = EncryptedObject::new();
    let output = encrypted_object.serialize().unwrap();

    File::create("eter/tests/data/mock_crypted_object.epk")
        .expect("Unable to create file")
        .write_all(&output[1..])
        .expect("Could not write to file");


    // TODO: Meta information should be emitted here, which is what EncryptedObject
    //       should do, and why we want the replacement of TypeRaw, probably
    assert_eq!(&output[1..], b"MCOZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    */
}

#[test]
fn load_encrypted_object() {
    let path = Path::new(MOCK_ENCRYPTED_OBJECT);
    let file = fs::read(path).unwrap();

    // let packer = TypeRaw::with_key(Key::default());
    let packer = TypeRaw::with_properties(
        Key::default(), *MCOZ_FOURCC, *MCSP_FOURCC
    );

    let encrypted_object = packer.deserialize(file, true).unwrap();

    dbg!(encrypted_object);
}
*/
