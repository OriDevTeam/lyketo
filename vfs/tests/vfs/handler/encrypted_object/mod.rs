/*
// Relative Modules
mod index;

// Standard Uses

// Crate Uses

// External Uses
use lazy_static::lazy_static;
use lyketo_vfs::utils::key::Key;


// pub const PATH: &str = "tests/data/eterfs/test.mock";

lazy_static!(
    static ref DEFAULT_COMPRESSION_KEY: Key = Key::from_segments_u32(
        vec![45129401, 92367215, 681285731, 1710201]
    );

    static ref DEFAULT_SECURITY_KEY: Key = Key::from_segments_u32(
        vec![78952482, 527348324, 1632942, 486274726]
    );
);


#[test]
pub fn create_handler () {
    /*
    let vfs: Box<dyn FileSystem> = Box::new(Eter::new());

    let mut handler = EncryptedObject::new(vfs);
    handler.compression_key.set_u32(DEFAULT_COMPRESSION_KEY);

    let pack = Box::new(Pack::load_from_disk(PATH).unwrap());
    handler.add_unit(pack).unwrap();
    */

    todo!()
}
*/

