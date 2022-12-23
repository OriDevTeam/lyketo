// Standard Uses
use std::mem;

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use anyhow::{bail, Result};
use extended_tea::XTEA as ExtendedTEA;
use byteorder::LE;


#[derive(Debug, Default)]
pub struct XTEA {}

impl Cipher for XTEA {
    fn encrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        if key.len() != mem::size_of::<[u32; 4]>() {
            bail!("Expected {} bytes for key, got {} instead",
                mem::size_of::<[u32; 4]>(), key.len()
            );
        }

        let concrete_key = bytemuck::from_bytes(&key);

        let cipher = ExtendedTEA::new(concrete_key);

        let mut output = vec![0u8; data.len()].into_boxed_slice();
        cipher.encipher_u8slice::<LE>(&data, &mut output);

        Ok(Vec::from(output))
    }

    fn decrypt(data: &Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        if key.len() != mem::size_of::<[u32; 4]>() {
            bail!("Expected {} bytes for key, got {} instead",
                mem::size_of::<[u32; 4]>(), key.len()
            );
        }

        let concrete_key = bytemuck::from_bytes(&key);

        let cipher = ExtendedTEA::new(concrete_key);

        let mut output = vec![0u8; data.len()].into_boxed_slice();
        cipher.decipher_u8slice::<LE>(&data, &mut output);

        Ok(Vec::from(output))
    }
}
