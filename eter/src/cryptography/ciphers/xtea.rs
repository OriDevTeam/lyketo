// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use extended_tea::XTEA as ExtendedTEA;
use byteorder::LE;


const ETER_INDEX_KEY: [u32; 4] = [0xB99EB002, 0x6F698105, 0x63989B28, 0x79181A00];
const _ETER_PACK_KEY: [u32; 4] = [0x22B8B404, 0x64B26E1F, 0xAEEA1800, 0xA6F6FB1C];


#[derive(Debug, Default)]
pub struct XTEA {}

impl Cipher for XTEA {
    fn encrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let cipher = ExtendedTEA::new(&ETER_INDEX_KEY);

        let mut output = vec![0u8; data.len()].into_boxed_slice();
        cipher.encipher_u8slice::<LE>(&data, &mut output);

        Ok(Vec::from(output))
    }

    fn decrypt(data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let cipher = ExtendedTEA::new(&ETER_INDEX_KEY);

        let mut output = vec![0u8; data.len()].into_boxed_slice();
        cipher.decipher_u8slice::<LE>(&data, &mut output);

        Ok(Vec::from(output))
    }
}
