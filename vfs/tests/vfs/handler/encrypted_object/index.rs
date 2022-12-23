// Standard Uses

// Crate Uses

// External Uses


// const TEST_PACK_UNIT_VERSION2_CRC_INDEX: &str = "tests/data/eterfs_m2/BGM.eix";
// const TEST_PACK_UNIT_VERSION2_CRC_DATA: &str = "tests/data/eterfs_m2/BGM.epk";

// const TEST_PACK_UNIT_VERSION2_MD5_INDEX: &str = ""; // TODO: Make unit files with MD5
// const TEST_PACK_UNIT_VERSION2_MD5_DATA: &str = ""; // TODO: Make unit files with MD5


#[test]
pub fn load_version2_crc() {
    /*
    let index_file = fs::read(TEST_PACK_UNIT_VERSION2_CRC_INDEX).unwrap();

    let header = (&index_file[0..16]).try_into().unwrap();
    let files = index_file[16..].to_vec();

    let index_header = Header::from_bytes(header).unwrap();

    let index_files = load_files_information(
        files, index_header.index_count as usize
    );

    assert_eq!(index_header.index_count as usize, index_files.len());
    */

    todo!()
}

#[test]
pub fn load_version2_md5() {
    todo!()
}

