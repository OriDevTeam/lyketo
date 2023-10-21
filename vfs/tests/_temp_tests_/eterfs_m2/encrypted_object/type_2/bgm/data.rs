// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses
use crate::_temp_tests_::eterfs_m2::encrypted_object::type_2::bgm::BGM_DATA_PATH;

// External Uses
use lyketo_vfs::formats::encrypted_object::types::type_2::ETER_DATA_KEY;
use lyketo_vfs::formats::encrypted_object::types::Type2;
use lyketo_vfs::formats::MCOZ_FOURCC;


#[test]
pub fn load_data_deserialize() {
    let path = Path::new(BGM_DATA_PATH);
    let file = fs::read(path).unwrap();

    // let packer = Type2::with_key(ETER_DATA_KEY.clone());
    let packer = Type2::with_properties(
        ETER_DATA_KEY.clone(), *MCOZ_FOURCC, *MCOZ_FOURCC
    );

    // let object_data = packer.deserialize(file, true).unwrap();

    // assert_eq!(EXPECTED_DATA_DECOMPRESSED_DATA, index_data.as_slice());
}

