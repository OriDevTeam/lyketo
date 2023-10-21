// Standard Uses

// Crate Uses
use crate::cryptography::{ciphers, compressors};
use crate::cryptography::ciphers::xtea::XTEA;
use crate::cryptography::compressors::lzo_mini::LZOMini;
use crate::cryptography::compressors::snappy::Snappy;
use crate::formats::encrypted_object::EncryptedObject;

// External Uses


pub type TypeRaw = EncryptedObject<compressors::None, ciphers::None>;
pub type Type1 = EncryptedObject<LZOMini, ciphers::None>;
pub type Type2 = EncryptedObject<LZOMini, XTEA>;
pub type Type6 = EncryptedObject<Snappy, XTEA>;


pub mod type_1 {}

pub mod type_2 {
    // Crate Uses
    use crate::utils::key::Key;

    // External Uses
    use once_cell::sync::Lazy;


    pub static ETER_INDEX_KEY: Lazy<Key> = Lazy::new(||Key::from_segments_u32(
        vec![45129401, 92367215, 681285731, 1710201]
    ));

    pub static ETER_DATA_KEY: Lazy<Key> = Lazy::new(||Key::from_segments_u32(
        vec![78952482, 527348324, 1632942, 486274726]
    ));

}

pub mod type_3 {}

