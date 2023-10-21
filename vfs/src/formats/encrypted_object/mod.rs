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
use crate::utils::key::Key;
use crate::utils::{four_cc, four_cc::FourCC};

// External Uses
use bytemuck;
use anyhow::{Result, bail};


pub struct EncryptedObject<Compression, Cipher> {
    compression: PhantomData<Compression>,
    cipher: PhantomData<Cipher>,

    cipher_key: Option<Key>,
    compression_magic: Option<FourCC>,
    cipher_magic: Option<FourCC>
}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {
    // TODO: When doing with_key, we could possibly try to detect the magic/fourcc
    //       that should be checked and used, check if this idea works
    /*
    pub fn with_key(cipher_key: Key) -> Self {
        Self {
            compression: Default::default(),
            cipher: Default::default(),
            cipher_key: Some(cipher_key),
            compression_magic: None,
            cipher_magic: None
        }
    }
    */

    pub fn with_properties(
        cipher_key: Key, compression_magic: FourCC, cipher_magic: FourCC
    ) -> Self {
        Self {
            compression: Default::default(),
            cipher: Default::default(),
            cipher_key: Some(cipher_key),
            compression_magic: Some(compression_magic),
            cipher_magic: Some(cipher_magic)
        }
    }

    pub fn deserialize(&self, data: Vec<u8>, log: bool) -> Result<Vec<u8>> {
        if data.len() < mem::size_of::<Header>() {
            bail!("Expected at least {} bytes, got {} instead",
                mem::size_of::<Header>(), &data.len()
            )
        }

        if log {
            println!(
                "Encrypted Object Data (size is {} bytes, first 10 bytes): {:?}({})",
                data.len(), &data[..10], String::from_utf8_lossy(&data[..10])
            );
        }

        let header = Header::from_bytes(data[..mem::size_of::<Header>()].to_vec())?;

        /*
        // TODO: This was here to theory check if we need to get the first 4 bytes(magic), pad them
        //       to multiple of 8 with 0's and the decipher, but might no be necessary so can be
        //       removed if confirmed it isn't
        let mut possible_fourcc = data[
            mem::size_of::<Header>()..mem::size_of::<Header>() + 4
        ].to_vec();
        possible_fourcc.resize(8, 0u8);

        let c = Ciphering::decrypt(
            &possible_fourcc, self.cipher_key.as_ref().unwrap().to_vec_u8()
        ).unwrap();

        println!("{:?}({})", c, String::from_utf8_lossy(&c));
        */

        let object_data = &data[mem::size_of::<Header>()..].to_vec();

        if log {
            header.print_pre_deciphering_info()
        };

        if header.compression_magic != self.compression_magic.unwrap() {
            bail!(
                "Expected Compression FourCC {}({}), but got {}({}) instead",
                self.compression_magic.unwrap(), four_cc::to_string(&self.compression_magic.unwrap()),
                header.compression_magic, four_cc::to_string(&header.compression_magic)
            );
        }

        if log {
            println!("Object Data Without Header (size is {} bytes, first 10 bytes): {:?}({})",
                     &object_data.len(), &object_data[..10],
                     String::from_utf8_lossy(&object_data[..10])
            );
        }

        let mut deciphered_data = if header.ciphered_size > 0 {

            if object_data.len() != header.ciphered_size as usize {
                bail!("Object data without Header size is {}, should be {}",
                object_data.len(), header.ciphered_size
            );
            }

            let deciphered_data = Ciphering::decrypt(
                &object_data, self.cipher_key.as_ref().unwrap().to_vec_u8()
            )?;

            if log {
                println!(
                    "Deciphered with {} ({} bytes, first 10 bytes): {:?}({})",
                    Ciphering::NAME,
                    deciphered_data.len(), &deciphered_data[..10],
                    String::from_utf8_lossy(&deciphered_data[..10])
                );
            }

            let four_cc_bytes: [u8; 4] = *bytemuck::from_bytes(&deciphered_data[..4]);
            let compression_four_cc = four_cc::from_bytes(four_cc_bytes);

            if compression_four_cc != self.compression_magic.unwrap() {
                bail!("Expected deciphered data FourCC to be {}, got {} instead",
                    four_cc::to_string(&self.compression_magic.unwrap()),
                    four_cc::to_string(&compression_four_cc),
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

        if log {
            header.print_pre_decompression_info();
        }

        deciphered_data.drain(..4);
        println!("Drained into \n{:?}", &deciphered_data[deciphered_data.len() - 16..]);
        deciphered_data.resize(header.ciphered_size as usize, 0);
        println!("Resized into \n{:?}", &deciphered_data[deciphered_data.len() - 16..]);

        println!(
            "Compressed Object Data: ({} bytes, first 10 bytes) {:?}({})",
            deciphered_data.len(), &deciphered_data[..10],
            String::from_utf8_lossy(&deciphered_data[..10])
        );

        let decompressed_data = Compression::decompress(
            &deciphered_data, header.raw_size as usize
        ).unwrap();

        if decompressed_data.len() != header.raw_size as usize {
            bail!("Decompressed size is {}, should be {}",
                decompressed_data.len(), header.compressed_size
            );
        }

        if log {
            println!(
                "Decompressed {} bytes (first 10): {:?}({})",
                decompressed_data.len(), &decompressed_data[..10],
                String::from_utf8_lossy(&deciphered_data[..10])
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
            compression_magic: self.compression_magic.unwrap(),
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
