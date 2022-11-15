// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;
use crate::cryptography::ciphers::sha1::SHA1;
use crate::cryptography::ciphers::tiger::Tiger;
use crate::cryptography::ciphers::whirlpool::Whirlpool;
use crate::cryptography::ciphers::ripemd128::RIPEMD128;

// External Uses
use anyhow::{Result, bail};
use enum_dispatch::enum_dispatch;
use strum::EnumCount;
use strum_macros::{EnumIter};
use crc32fast;


pub struct Panama {
    cipher_key: [u32; 4]
}


#[allow(unused)]
impl Panama {
    pub fn with_key(cipher_key: [u32; 4]) -> Self {
        Self {
            cipher_key
        }
    }

    pub fn serialize(&self, filename: &str, data: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), ()>{
        // TODO: Very precise information is missing, this code is far from representing
        //       what actually should be happening, needs theory investigation

        let filename_crc = crc32fast::hash(filename.as_bytes());
        let filename_cipher_index = filename_crc & (FilenameCipher::COUNT as u32 - 1);

        let ciphered_filename = FilenameCipher::encrypt(
            filename_cipher_index, Vec::from(filename),
            bytemuck::cast::<[u32; 4], [u8; 16]>(self.cipher_key).to_vec()
        ).unwrap();

        let panama_key_iv: &[u8] = &ciphered_filename[0..4];
        let panama_key_num: u32 = u32::from_le_bytes(panama_key_iv.try_into().unwrap());
        let panama_cipher_index = panama_key_num & (PanamaKeyCipher::COUNT as u32 - 1);

        /*
        let panama_cipher_index = panama_key_iv.iter()
            .map(|x| x & PanamaKeyCipher::COUNT as u8)
            .collect::<Vec<_>>().to_owned();
           */

        let panama_key = PanamaKeyCipher::encrypt(
            panama_cipher_index, panama_key_iv.to_vec(),
            bytemuck::cast::<[u32; 4], [u8; 16]>(self.cipher_key).to_vec()
        ).unwrap();

        Ok((panama_key, data))
    }

    pub fn deserialize(&self, filename: &str, data: Vec<u8>) -> Result<Vec<u8>>{
        todo!()
    }

    pub fn set_key(&mut self, key: [u32; 4]) {
        self.cipher_key = key
    }
}


#[derive(Debug, EnumCount, EnumIter)]
#[enum_dispatch(Cipher)]
enum FilenameCipher {
    Whirlpool(Whirlpool),
    Tiger(Tiger),
    SHA1(SHA1),
    RIPEMD128(RIPEMD128)
}


impl FilenameCipher {
    fn encrypt(index: u32, data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        // let cipher = FilenameCipher::iter().nth((pathm - 1) as usize).unwrap();

        let result = match index - 1 {
            0 => Whirlpool::encrypt(&data, key),
            1 => Tiger::encrypt(&data, key),
            2 => SHA1::encrypt(&data, key),
            3 => RIPEMD128::encrypt(&data, key),
            _ => bail!("No cypher found for index {}", index)
        };

        result
    }
}



#[derive(Debug, EnumCount, EnumIter)]
#[enum_dispatch(Cipher)]
enum PanamaKeyCipher {
    SHA1(SHA1),
    RIPEMD128(RIPEMD128),
    Whirlpool(Whirlpool),
    Tiger(Tiger)
}

impl PanamaKeyCipher {
    fn encrypt(index: u32, data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>> {
        let result = match index - 1 {
            2 => Whirlpool::encrypt(&data, key),
            3 => Tiger::encrypt(&data, key),
            0 => SHA1::encrypt(&data, key),
            1 => RIPEMD128::encrypt(&data, key),
            _ => bail!("No cypher found for index {}", index)
        };

        result
    }
}

