// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use lyketo_vfs::formats::protos::item_proto::{ItemProto, stride_171};


const ITEM_PROTO_PATH: &str = "tests/temp/data/eterfs_m2/encrypted_object/type_2/locale/item_proto";


#[allow(unused)]
#[test]
fn load_file_parse_proto() {
    let file = fs::read(Path::new(ITEM_PROTO_PATH)).unwrap();

    // println!("{}", std::mem::size_of::<mipx::ItemTable>());

    let item_proto = ItemProto::from_bytes(file).unwrap();
}

