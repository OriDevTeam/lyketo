// Relative Modules
// mod type_1;
// mod type_2;
// mod panama;


// Standard Uses
// use std::fs::File;
// use std::io::{Read, Write};

// Crate Uses

// External Uses
// use lyketo_eter::formats::encrypted_object::EncryptedObject;



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
    /*
    let mut buffer = vec![];
    let _ = File::open("eter/tests/data/mock_encrypted_object.epk")
        .expect("{}")
        .read_to_end(&mut buffer)
        .expect("{}");

    let encrypted_object = EncryptedObject::from_bytes(buffer);

    encrypted_object.expect_err("{}")
    */

}

