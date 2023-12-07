// Standard Uses
use std::mem;

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use eyre::{bail, Result};
use extended_tea::XTEA as ExtendedTEA;
use byteorder::LE;
use crate::formats::MCOZ_FOURCC;


#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default)]
pub struct XTEA {}

impl Cipher for XTEA {
    const FOURCC: u32 = *MCOZ_FOURCC;
    const NAME: &'static str = "XTEA";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        if key.len() != mem::size_of::<[u32; 4]>() {
            bail!("Expected {} bytes for key, got {} instead",
                mem::size_of::<[u32; 4]>(), key.len()
            );
        }

        let concrete_key = bytemuck::from_bytes(&key);

        let cipher = ExtendedTEA::new(concrete_key);
        let mut output = vec![0u8; data.len()];

        if data.len() % 8 != 0 {
            bail!(
                "Could not cipher with XTEA: Input should be of length divisible by 8, \
                 it's {} instead",
                data.len() % 8
            )
        }

        cipher.encipher_u8slice::<LE>(data, &mut output);
        Ok(output.to_vec())
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        if key.len() != mem::size_of::<[u32; 4]>() {
            bail!("Expected {} bytes for key, got {} instead",
                mem::size_of::<[u32; 4]>(), key.len()
            );
        }

        let concrete_key = bytemuck::from_bytes(&key);

        let cipher = ExtendedTEA::new(concrete_key);

        let mut output = vec![0u8; data.len()].into_boxed_slice();
        cipher.decipher_u8slice::<LE>(data, &mut output);

        Ok(output.to_vec())
    }
}
