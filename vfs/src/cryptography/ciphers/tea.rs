// Standard Uses
use std::mem;

// Crate Uses
use crate::cryptography::ciphers::Cipher;

// External Uses
use anyhow::{bail, Result};
use tea_soft::block_cipher::generic_array::GenericArray;
use tea_soft::block_cipher::{BlockCipher, NewBlockCipher};
use tea_soft::Tea32;


#[derive(Debug, Default)]
pub struct TEA {}


impl Cipher for TEA {
    const NAME: &'static str = "TEA";

    fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        if key.len() != mem::size_of::<[u8; 4]>() {
            bail!("Expected {} bytes, got {} instead", mem::size_of::<[u8; 4]>(), key.len());
        }

        if data.len() % 8 != 0 {
            bail!("Expected data to be the length of a multiple of 8, got {} instead", data.len());
        }

        // let concrete_key = &std::array::from_fn(|i| key[i] as u32).into();

        let key = GenericArray::from_slice(&key[0..16]);

        let cipher = Tea32::new(&key);

        let mut ciphered_data = Vec::with_capacity(data.len());

        let mut idx = 0;
        while idx < data.len() {
            let mut block = GenericArray::clone_from_slice(
                &data[idx..idx + 8]
            );

            cipher.encrypt_block(&mut block);

            ciphered_data.extend_from_slice(&block);

            idx += 8;
        }

        Ok(ciphered_data)
    }

    fn decrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
        if data.len() % 8 != 0 {
            bail!("Expected data to be the length of a multiple of 8, got {} instead", data.len());
        }

        // let concrete_key = &std::array::from_fn(|i| key[i] as u32).into();

        let key = GenericArray::from_slice(&key[0..16]);

        let cipher = Tea32::new(&key);

        let mut deciphered_data= Vec::with_capacity(data.len());

        let mut idx = 0;
        while idx < data.len() {
            let mut block = GenericArray::clone_from_slice(
                &data[idx..idx + 8]
            );

            cipher.decrypt_block(&mut block);
            deciphered_data.append(&mut block.to_vec());

            idx += 8;
        }

        Ok(deciphered_data)
    }
}
