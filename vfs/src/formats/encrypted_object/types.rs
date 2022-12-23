// Relative Modules
pub mod type_3;

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

