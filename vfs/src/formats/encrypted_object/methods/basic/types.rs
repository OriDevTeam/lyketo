// Standard Uses

// Crate Uses
use crate::cryptography::{ciphers, compressors};
use crate::cryptography::ciphers::xtea::XTEA;
use crate::cryptography::compressors::lzo_mini::LZOMini;
use crate::cryptography::compressors::snappy::Snappy;
use crate::formats::encrypted_object::methods::basic::EncryptedObject;
use crate::utils::four_cc::FourCC;

// External Uses
use eyre::Result;


pub type TypeRaw = EncryptedObject<ciphers::None, compressors::None>;
pub type Type1 = EncryptedObject<ciphers::None, LZOMini>;
pub type Type2 = EncryptedObject<XTEA, LZOMini>;
pub type Type6 = EncryptedObject<XTEA, Snappy>;


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

pub enum EterBasicType {
    TypeRaw(TypeRaw),
    Type1(Type1),
    Type2(Type2),
    Type6(Type6),
}

impl EterBasicType {
    pub fn deserialize(&self, data: Vec<u8>, log: bool) -> Result<Vec<u8>> {
        match self {
            EterBasicType::TypeRaw(r) => r.deserialize(data, log),
            EterBasicType::Type1(t1) => t1.deserialize(data, log),
            EterBasicType::Type2(t2) => t2.deserialize(data, log),
            EterBasicType::Type6(t6) => t6.deserialize(data, log),
        }
    }
}

pub struct Magic(FourCC);
