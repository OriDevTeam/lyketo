// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use etervfs::formats::mob_proto::Mobs;


const MOB_PROTO_PATH: &str = "tests/data/eterfs_m2/locale/mob_proto";


#[allow(unused)]
#[test]
fn load_file_parse_mmpt_proto() {
    let file = fs::read(Path::new(MOB_PROTO_PATH)).unwrap();
    let mob_proto = Mobs::from_bytes(file);

}

