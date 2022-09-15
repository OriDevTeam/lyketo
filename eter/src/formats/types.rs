// Relative Modules
pub mod type_3;

// Standard Uses

// Crate Uses
use crate::cryptography::ciphers;
use crate::cryptography::ciphers::xtea::XTEA;
use crate::cryptography::compressors::lzo::LZOMini;
use crate::cryptography::compressors::snappy::Snappy;
use crate::formats::encrypted_object::EncryptedObject;

// External Uses


pub type Type1 = EncryptedObject<LZOMini, ciphers::None>;
pub type Type2 = EncryptedObject<LZOMini, XTEA>;
pub type Type6 = EncryptedObject<Snappy, XTEA>;

