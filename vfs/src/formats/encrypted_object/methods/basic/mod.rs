// Relative Modules
pub mod types;

// Relative Modules
use std::mem;
use std::marker::PhantomData;

// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;
use crate::cryptography::ciphers::Cipher;
use crate::formats::encrypted_object::header::Header;
use crate::formats::encrypted_object::methods::basic::types::{EterBasicType, Type2};
use crate::utils::key::Key;
use crate::utils::{four_cc, four_cc::FourCC};

// External Uses
use bytemuck;
use eyre::{Result, bail};


pub struct EncryptedObject<Cipher, Compressor> {
    cipher: PhantomData<Cipher>,
    // cipher_magic: Option<FourCC>,
    cipher_key: Option<Key>,
    compression: PhantomData<Compressor>,
    // compression_magic: Option<FourCC>,
}


/*
type AnyType = (FourCC, FourCC);

pub fn guess_type(key: Key, data: Vec<u8>, log: bool) -> Result<AnyType> {
    if data.len() < mem::size_of::<Header>() {
        bail!(
            "Expected object data with at least {} bytes, got {} instead",
            mem::size_of::<Header>(), &data.len()
        )
    }

    let header = Header::from_bytes(data[..mem::size_of::<Header>()].to_vec())?;

    let cipher_magic = 0;
    let compressed_magic = 0;

    Ok((cipher_magic, compressed_magic))
}
*/

pub fn guess_type_and_deserialize(
    key: Key, data: Vec<u8>, log: bool
) -> Result<(Vec<u8>, EterBasicType)> {
    if data.len() < mem::size_of::<Header>() {
        bail!(
            "Expected object data with at least {} bytes, got {} instead",
            mem::size_of::<Header>(), &data.len()
        )
    }

    let header = Header::from_bytes(data[..mem::size_of::<Header>()].to_vec())?;

    let (deserialized, type_handler) = match header.ciphering_magic {
        10 => {
            let handler = EterBasicType::Type2(Type2::with_key(key));
            let deserialized = handler.deserialize(data, log)?;

            (deserialized, handler)
        }
        other => {
            bail!("")
        }
    };

    Ok((deserialized, type_handler))
}

impl<Ciphering: Cipher, Compression: Compressor> EncryptedObject<Ciphering, Compression> {
    const COMPRESSION_FOURCC: FourCC = Compression::FourCC;
    const CIPHERING_FOURCC: FourCC = Ciphering::FourCC;

    pub fn with_key(cipher_key: Key) -> Self {
        Self {
            compression: Default::default(),
            cipher: Default::default(),
            cipher_key: Some(cipher_key),
        }
    }

    pub fn deserialize(&self, data: Vec<u8>, log: bool) -> Result<Vec<u8>> {
        if data.len() < mem::size_of::<Header>() {
            bail!(
                "Expected object data with at least {} bytes, got {} instead",
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

        let object_data = &data[mem::size_of::<Header>()..].to_vec();

        if log {
            header.print_pre_deciphering_info()
        };

        if header.ciphering_magic != Ciphering::FOURCC {
            bail!(
                "Expected Ciphering FourCC to be {}({}), but got {}({}) instead",
                self.cipher_magic.unwrap(), four_cc::to_string(&self.cipher_magic.unwrap()),
                header.ciphering_magic, four_cc::to_string(&header.ciphering_magic)
            );
        }

        if log {
            println!(
                "Object Data Without Header (size is {} bytes, first 10 bytes): {:?}({})",
                &object_data.len(), &object_data[..10],
                String::from_utf8_lossy(&object_data[..10])
            );
        }

        let mut deciphered_data = if header.ciphered_size > 0 {
            self.decrypt(
                object_data, header.ciphered_size as usize, log
            )?
        } else {
            // TODO: This cannot be a clone, this clones way too much data non-efficiently
            object_data.to_vec()
        };

        if log { header.print_pre_decompression_info(); }

        deciphered_data.drain(..4);
        if log { println!("Drained into \n{:?}", &deciphered_data[deciphered_data.len() - 16..]); }

        deciphered_data.resize(header.compressed_size as usize, 0);
        if log { println!("Resized into \n{:?}", &deciphered_data[deciphered_data.len() - 16..]); }

        println!(
            "Compressed Object Data: ({} bytes, first 10 bytes) {:?}({})",
            deciphered_data.len(), &deciphered_data[..10],
            String::from_utf8_lossy(&deciphered_data[..10])
        );

        let decompressed_data = self.decompress(
            &deciphered_data, &header, log
        )?;

        Ok(decompressed_data)
    }

    pub fn decrypt(
        &self, encrypted_data: &[u8], ciphered_size: usize, log: bool
    ) -> Result<Vec<u8>> {
        let padding = encrypted_data.len() % 8;
        let expected_size_with_padding = ciphered_size - padding;
        let expected_size_cipher_relative_padding = ciphered_size + padding;

        let data_ref = if encrypted_data.len() == ciphered_size {
            &encrypted_data[..]

        } else if encrypted_data.len() == expected_size_with_padding {
            &encrypted_data[..]

        } else if encrypted_data.len() == expected_size_cipher_relative_padding {
            if log {
                println!(
                    "Ciphered data seems to have extra {} bytes padding, trimming to {}",
                    padding, ciphered_size
                );
            }

            &encrypted_data[..ciphered_size]
        } else {
            bail!(
                "Object data without Header size '{}' is not expected, it should be either:\n\
                 - {} (possibly padded relative to compressed size)\n\
                 - {} (without padding relative to compressed size)\n\
                 - {} (with padding relative to ciphered size)\
                 ",
                encrypted_data.len(), ciphered_size,
                expected_size_with_padding, expected_size_cipher_relative_padding
            );
        };

        let deciphered_data = Ciphering::decrypt(
            data_ref, self.cipher_key.as_ref().unwrap().to_vec_u8()
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

        if deciphered_data.len() != ciphered_size {
            bail!("Deciphered size is {}, should be {}",
                deciphered_data.len(), ciphered_size
            );
        }

        return Ok(deciphered_data)
    }

    fn decompress(&self, compressed_data: &[u8], header: &Header, log: bool) -> Result<Vec<u8>> {
        let decompressed_data = Compression::decompress(
            compressed_data, header.raw_size as usize
        )?;

        if decompressed_data.len() != header.raw_size as usize {
            bail!("Decompressed size is {}, should be {}",
                decompressed_data.len(), header.compressed_size
            );
        }

        if log {
            println!(
                "Decompressed {} bytes (first 10): {:?}({})",
                decompressed_data.len(), &decompressed_data[..10],
                String::from_utf8_lossy(&compressed_data[..10])
            );
        }

        Ok(decompressed_data)
    }
}


impl<Ciphering: Cipher, Compression: Compressor> EncryptedObject<Ciphering, Compression> {
    pub fn serialize(&self, data: Vec<u8>, log: bool) -> Result<Vec<u8>> {
        if log {
            println!("Raw object ({} bytes): {:?}", data.len(), String::from_utf8_lossy(&data));
        }

        if log { println!("Compressing with {}", Compression::NAME); }

        let (mut compressed_data, compressed_pre_pad_size) = self.compress(
            &data, log
        )?;

        // The data should be padded to multiple of 8 for the ciphers
        // Example if the data is 113 bytes we pad it to 120 bytes
        pad_into(&mut compressed_data, 8, log);

        if log { println!("Ciphering with {}", Ciphering::NAME); }

        let (ciphered_data, _) = self.encrypt(
            &compressed_data, log
        )?;

        // Create the header with all the necessary information
        // and push it, then the ciphered object after
        let mut object: Vec<u8> = vec!();

        let header = Header {
            ciphering_magic: self.compression_magic.unwrap(),
            ciphered_size: ciphered_data.len() as _,
            compressed_size: compressed_pre_pad_size as _,
            raw_size: data.len() as _,
        };

        object.extend(bytemuck::bytes_of(&header));
        object.extend(ciphered_data.to_owned());

        Ok(object)
    }

    fn encrypt(&self, raw_data: &[u8], log: bool) -> Result<(Vec<u8>, usize)> {
        let mut ciphered_data = Ciphering::encrypt(
            raw_data, self.cipher_key.as_ref().unwrap().to_vec_u8()
        )?;

        if log {
            println!(
                "Ciphered with {} (into {} bytes, first 10): {:?}({})",
                Ciphering::NAME, ciphered_data.len(), &ciphered_data[..10],
                String::from_utf8_lossy(&ciphered_data[..10])
            );
            println!("Whole ciphered data: {:?}", &ciphered_data);
            println!(
                "Whole ciphered data (as String): {:?}",
                String::from_utf8_lossy(&ciphered_data)
            );
        }

        // The data should be padded to multiple of 8 for the ciphers
        // Example if the data is 113 bytes we pad it to 120 bytes
        let pre_padding_size = ciphered_data.len();
        pad_into(&mut ciphered_data, 8, log);

        Ok((ciphered_data, pre_padding_size))
    }

    fn compress(&self, ciphered_data: &[u8], log: bool) -> Result<(Vec<u8>, usize)> {
        let mut compressed_data = Compression::compress(ciphered_data)?;

        if log {
            println!(
                "Compressed with {} (into {} bytes, first 10): {:?}({})",
                Compression::NAME, compressed_data.len(), &compressed_data[..10],
                String::from_utf8_lossy(&compressed_data[..10])
            );

            println!("Whole compressed data: {:?}", &compressed_data);
            println!(
                "Whole compressed data (as String): {:?}",
                String::from_utf8_lossy(&compressed_data)
            );
        }

        let pre_pad_size = compressed_data.len();

        let compression_fourcc = four_cc::to_bytes(self.compression_magic.unwrap());
        compressed_data.splice(0..0, compression_fourcc);

        if log {
            println!(
                "Added compression FourCC/Magic '{}' to the start of compressed data\
                 (first 10 bytes): {:?}({})",
                four_cc::to_string(Compressor::FOURCC),
                &compressed_data[..10], String::from_utf8_lossy(&compressed_data[..10])
            )
        }

        Ok((compressed_data, pre_pad_size))
    }

}

fn pad_into(data: &mut Vec<u8>, div_by: usize, log: bool) {
    let mut padding: usize = 0;

    if data.len() % div_by != 0 {
        padding = (data.len() + div_by) / div_by * div_by;
    }

    if padding != 0 {
        if log {
            println!(
                "Padding to multiple of 8 at the right, from {} to {} bytes",
                data.len(), padding
            );
        }

        data.resize(padding, 0);

        // TODO: Show last 10 bytes here
        // if log { println!("{:?}", ciphered_data); }
    }
}


