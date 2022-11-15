// Standard Uses

// Crate Uses

// External Uses

// Standard Uses

// Crate Uses
use crate::vfs::eterfs::CRC32;
use crate::vfs::handler::encrypted_object::index::FILE_NAME_LEN;

// External Uses

#[repr(align(4))]
pub struct IndexMD5 {
    pub id: i32,
    pub file_name: [u8; FILE_NAME_LEN + 1],
    pub file_crc: CRC32, // TODO: replace with MD5 instead
    pub raw_data_size: i32,
    pub processed_data_size: i32,
    pub processed_data_crc: CRC32,
    pub processed_data_position: i32,
    pub compression_type: i8
}

