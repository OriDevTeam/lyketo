// Relative Modules
pub mod header;
pub mod types;

// Relative Modules
use std::mem;
use std::marker::PhantomData;

// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;
use crate::cryptography::ciphers::Cipher;
use crate::formats::encrypted_object::header::Header;
use crate::utils::four_cc;
use crate::utils::key::Key;

// External Uses
use bytemuck;
use anyhow::{Result, bail};
use lazy_static::lazy_static;


lazy_static!(
    pub static ref ETER_INDEX_KEY: Key = Key::from_decimals_u32(
        vec![45129401, 92367215, 681285731, 1710201]
    );

    pub static ref ETER_DATA_KEY: Key = Key::from_decimals_u32(
        vec![78952482, 527348324, 1632942, 486274726]
    );
);


pub struct EncryptedObject<Compression, Cipher> {
    compression: PhantomData<Compression>,
    cipher: PhantomData<Cipher>,

    cipher_key: Option<Key>
}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {
    pub fn with_key(cipher_key: Key) -> Self {
        Self {
            compression: Default::default(),
            cipher: Default::default(),
            cipher_key: Some(cipher_key)
        }
    }

    pub fn deserialize(&self, data: Vec<u8>, log: bool) -> Result<Vec<u8>> {
        if data.len() < mem::size_of::<Header>() {
            bail!("Expected at least {} bytes, got {} instead",
                mem::size_of::<Header>(), &data.len()
            )
        }

        if log { println!("Encrypted Object size: {} bytes", data.len()); }

        let header = Header::from_bytes(data[..mem::size_of::<Header>()].to_vec())?;
        let object_data = &data[mem::size_of::<Header>()..].to_vec();

        header.print_information(object_data, log);

        if header.compression_magic != Compression::FOURCC {
            bail!("Compression FourCC is not {}", Compression::FOURCC);
        }

        if log {
            println!("Processed Object Data ({} bytes, first 10 bytes): {:?}",
                 &object_data.len(), &object_data[..10]
            );
        }

        let mut pre_compressed_data = if header.ciphered_size > 0 {

            let deciphered_data = Ciphering::decrypt(
                &object_data, self.cipher_key.as_ref().unwrap().to_vec_u8()
            )?;

            if log {
                println!("Deciphered ({} bytes, first 10 bytes): {:?}",
                         deciphered_data.len(), &deciphered_data[..10]
                );
            }

            let four_cc_bytes: [u8; 4] = *bytemuck::from_bytes(&deciphered_data[..4]);
            let deciphered_four_cc = four_cc::from_bytes(four_cc_bytes);

            if deciphered_four_cc != Compression::FOURCC {
                bail!("Expected deciphered data FourCC to be {}, got {} instead",
                    four_cc::to_string(&Compression::FOURCC),
                    four_cc::to_string(&deciphered_four_cc),
                );
            }

            if deciphered_data.len() != header.ciphered_size as usize {
                bail!("Deciphered size is {}, should be {}",
                    deciphered_data.len(), header.ciphered_size
                );
            }

            deciphered_data
        } else {
            // TODO: This cannot be a clone, this clones way too much data non-efficiently
            // FIXME: It should either move the object or reference it, change the flow of the let
            object_data.clone()
        };

        pre_compressed_data.drain(..4);
        pre_compressed_data.resize(header.compressed_size as usize, 0);

        println!("Compressed Object Data: ({} bytes, first 10 bytes) {:?}",
                 pre_compressed_data.len(), &pre_compressed_data[..10]
        );

        let decompressed_data = Compression::decompress(
            &pre_compressed_data, header.raw_size as usize
        ).unwrap();

        if decompressed_data.len() != header.raw_size as usize {
            bail!("Decompressed size is {}, should be {}",
                decompressed_data.len(), header.compressed_size
            );
        }

        if log {
            println!("Decompressed {} bytes (first 10): {:?}",
                     decompressed_data.len(), &decompressed_data[..10]
            );
        }

        Ok(decompressed_data)
    }

}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {

    pub fn serialize(&self, data: Vec<u8>) -> Result<Vec<u8>> {

        println!("Raw object size: {}", data.len());
        println!("{:?}", &data);


        let mut compressed_data = Compression::compress(&data).unwrap();
        let compressed_data_pre_padding = compressed_data.len();

        println!("Compressed from {} to {} bytes", data.len(),
                 compressed_data.len()
        );
        println!("{:?}", &compressed_data);

        let mut padding: usize = 0;
        // The data should be padded to multiple of 8 for the ciphers
        // Example if the data is 113 bytes we pad it to 120 bytes
        if compressed_data.len() % 8 != 0 {
            padding = (compressed_data.len() + 8) / 8 * 8;
        }

        if padding != 0 {
            println!("Padding to multiple of 8 at the right, from {} to {} bytes",
                     compressed_data.len(), padding);

            compressed_data.resize(padding, 0);

            println!("{:?}", compressed_data);
        }

        let ciphered_data = Ciphering::encrypt(
            &compressed_data, self.cipher_key.as_ref().unwrap().to_vec_u8()
        ).unwrap();

        println!("Ciphered from {} to {} bytes", compressed_data.len(),
                 ciphered_data.len());
        println!("{:?}", &ciphered_data);

        println!("Processed Object Data Size: {:?}", compressed_data.len());

        // Create the header with all the necessary information
        // and push it, then the ciphered object after
        let mut object: Vec<u8> = vec!();

        let header = Header {
            compression_magic: Compression::FOURCC,
            ciphered_size: ciphered_data.len() as _,
            compressed_size: compressed_data_pre_padding as _,
            raw_size: data.len() as _,
            // cipher_magic: Compression::FOURCC
        };

        object.extend(bytemuck::bytes_of(&header));
        object.extend(ciphered_data.to_owned());

        Ok(object)
    }
}
