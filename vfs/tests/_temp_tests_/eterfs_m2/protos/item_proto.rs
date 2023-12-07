// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use lyketo_vfs::formats::protos::item_proto::{ItemProto, stride_199};
use lyketo_vfs::formats::MIPX_FOURCC;


const ITEM_PROTO_PATH: &str = "tests/__temp_data__/eterfs_m2/encrypted_object/type_2/locale/item_proto";


#[allow(unused)]
#[test]
fn load_file_parse_proto() {
    let file = fs::read(Path::new(ITEM_PROTO_PATH)).unwrap();

    let item_proto = ItemProto::<stride_199::Record>::from_bytes_with_properties(
        file, *MIPX_FOURCC
    ).unwrap();

    println!("IP");
}

