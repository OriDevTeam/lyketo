/*
// Standard Uses
use std::fs;
use std::path::Path;


// Crate Uses

// External Uses
use lyketo_vfs::formats::protos::mob_proto::Mobs;
use lyketo_vfs::formats::encrypted_object::ETER_INDEX_KEY;
use lyketo_vfs::formats::encrypted_object::types::Type6;


const MOB_PROTO_PATH: &str = "tests/__temp_data__/eterfs_m2/encrypted_object/type_2/locale/\
                              mob_proto";


#[allow(unused)]
#[test]
fn load_file_parse_mmpt_proto() {
    let file = fs::read(Path::new(MOB_PROTO_PATH)).unwrap();

    println!("{:?}", file[..10].to_vec());

    let handler = Type6::with_key(ETER_INDEX_KEY.clone());
    let mob_proto = Mobs::from_bytes(file).unwrap();
}
*/
