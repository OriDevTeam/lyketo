// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::Cipher;
use crate::cryptography::ciphers::sha1::SHA1;
use crate::cryptography::ciphers::tiger::Tiger;
use crate::cryptography::ciphers::whirlpool::Whirpool;
use crate::cryptography::ciphers::ripemd128::RIPEMD128;

// External Uses
use crc32fast;
use enum_dispatch::enum_dispatch;
use strum::EnumCount;
use strum_macros::{EnumIter};



pub struct Panama {
}

impl Panama {

    pub fn serialize(filename: &str, data: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), ()>{
        // TODO: Very precise information is missing, this code is far from representing
        //       what actually should be happening, needs theory investigation

        let filename_crc = crc32fast::hash(filename.as_bytes());
        let filename_cipher_index = filename_crc & (FilenameCipher::COUNT as u32 - 1);

        let ciphered_filename = FilenameCipher::encrypt(filename_cipher_index, Vec::from(filename))
            .unwrap();

        let panama_key_iv: &[u8] = &ciphered_filename[0..4];
        let panama_key_num: u32 = u32::from_le_bytes(panama_key_iv.try_into().unwrap());
        let panama_cipher_index = panama_key_num & (PanamaKeyCipher::COUNT as u32 - 1);

        /*
        let panama_cipher_index = panama_key_iv.iter()
            .map(|x| x & PanamaKeyCipher::COUNT as u8)
            .collect::<Vec<_>>().to_owned();
           */

        let panama_key = PanamaKeyCipher::encrypt(panama_cipher_index, panama_key_iv.to_vec()).unwrap();

        Ok((panama_key, data))
    }

}

impl Panama {

    pub fn deserialize(_filename: &str, _data: Vec<u8>) -> Result<Vec<u8>, ()>{
        todo!()
    }

}


#[derive(Debug, EnumCount, EnumIter)]
#[enum_dispatch(Cipher)]
enum FilenameCipher {
    Whirlpool(Whirpool),
    Tiger(Tiger),
    SHA1(SHA1),
    RIPEMD128(RIPEMD128)
}


impl FilenameCipher {
    fn encrypt(index: u32, data: Vec<u8>) -> Result<Vec<u8> ,()> {
        // let cipher = FilenameCipher::iter().nth((index - 1) as usize).unwrap();

        let result = match index - 1 {
            0 => Whirpool::encrypt(&data),
            1 => Tiger::encrypt(&data),
            2 => SHA1::encrypt(&data),
            3 => RIPEMD128::encrypt(&data),
            _ => Err(())
        };

        result
    }
}



#[derive(Debug, EnumCount, EnumIter)]
#[enum_dispatch(Cipher)]
enum PanamaKeyCipher {
    SHA1(SHA1),
    RIPEMD128(RIPEMD128),
    Whirlpool(Whirpool),
    Tiger(Tiger)
}

impl PanamaKeyCipher {
    fn encrypt(index: u32, data: Vec<u8>) -> Result<Vec<u8> ,()> {
        let result = match index - 1 {
            2 => Whirpool::encrypt(&data),
            3 => Tiger::encrypt(&data),
            0 => SHA1::encrypt(&data),
            1 => RIPEMD128::encrypt(&data),
            _ => Err(())
        };

        result
    }
}

