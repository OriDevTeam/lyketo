// Standard Uses
use std::mem;

// Crate Uses
use crate::vfs::eterfs::CRC32;
use crate::vfs::eterfs::handlers::encrypted_object::index::FILE_NAME_LEN;

// External Uses



#[repr(align(4))]
pub struct IndexCRC {
    pub id: i32,
    pub file_name: [u8; FILE_NAME_LEN + 1],
    pub file_crc: CRC32,
    pub raw_data_size: i32,
    pub processed_data_size: i32,
    pub processed_data_crc: CRC32,
    pub processed_data_position: i32,
    pub compression_type: i8
}


#[allow(unused)]
impl IndexCRC {
    pub fn load(file: [u8; mem::size_of::<IndexCRC>()]) {
        todo!()
    }
}

