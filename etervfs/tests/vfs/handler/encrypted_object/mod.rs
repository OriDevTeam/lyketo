// Relative Modules
mod index;

// Standard Uses

// Crate Uses
use etervfs::vfs::eterfs::Eter;
use etervfs::vfs::FileSystem;
use etervfs::vfs::handler::encrypted_object::EncryptedObject;
use etervfs::vfs::handler::encrypted_object::pack::Pack;
use etervfs::vfs::handler::Handler;

// External Uses


const PATH: &str = "tests/data/eterfs/test.mock";

const DEFAULT_COMPRESSION_KEY: [u32; 4] = [45129401, 92367215, 681285731, 1710201];
// const DEFAULT_SECURITY_KEY: [u32; 4] = [78952482, 527348324, 1632942, 486274726];


#[test]
pub fn create_handler () {
    let vfs: Box<dyn FileSystem> = Box::new(Eter::new());

    let mut handler = EncryptedObject::new(vfs);
    handler.compression_key.set_u32(DEFAULT_COMPRESSION_KEY);

    let pack = Box::new(Pack::load_from_disk(PATH).unwrap());
    handler.add_unit(pack).unwrap();
}

