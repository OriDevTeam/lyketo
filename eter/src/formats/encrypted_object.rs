// Relative Modules
use std::marker::PhantomData;

// Standard Uses

// Crate Uses
use crate::cryptography::compressors::Compressor;
use crate::cryptography::ciphers::Cipher;

// External Uses



pub struct EncryptedObject<Compression, Cipher> {
    compression: PhantomData<Compression>,
    cipher: PhantomData<Cipher>,
}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {

    pub fn deserialize(data: Vec<u8>) -> Result<Vec<u8>, ()> {
        if data.len() < 20 {
            panic!("Not enough data for the header, expected at least 20 bytes, got {}",
                   &data.len())
        }

        println!("Encrypted Object size: {}", data.len());

        let compression_magic = &data[0..4];
        let ciphered_size = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let compressed_size = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let raw_size = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let cipher_magic = &data[16..20];
        let object = &data[16..].to_vec();

        Self::print_information(&compression_magic, &cipher_magic,
                                &object.len(), &ciphered_size,
                                &compressed_size, &raw_size);

        let compression_magic_name = String::from_utf8(Vec::from(compression_magic))
            .unwrap();

        if compression_magic_name != Compression::FOURCC {
            println!("Compression FourCC is not {}", Compression::FOURCC);
            return Err(())
        }

        println!("Processed Object ({} bytes): {:?}", &object.len(), &object);

        let mut deciphered_data = Ciphering::decrypt(&object).unwrap();
        println!("Deciphered ({} bytes): {:?}", deciphered_data.len(), deciphered_data);

        if deciphered_data.len() != ciphered_size as usize {
            println!("Deciphered size is {}, should be {}", deciphered_data.len(), ciphered_size);
            return Err(())
        }

        deciphered_data.truncate(compressed_size as usize);

        let decompressed_data = Compression::decompress(
            &deciphered_data, compressed_size as usize
        ).unwrap();

        println!("Decompressed ({} bytes): {:?}", decompressed_data.len(), decompressed_data);

        if decompressed_data.len() != raw_size as usize {
            println!("Decompressed size is {}, should be {}", decompressed_data.len(), compressed_size);
            return Err(())
        }

        Ok(decompressed_data)
    }

}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {

    pub fn serialize(data: Vec<u8>) -> Result<Vec<u8>, ()> {

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

        let ciphered_data = Ciphering::encrypt(&compressed_data).unwrap();

        println!("Ciphered from {} to {} bytes", compressed_data.len(),
                 ciphered_data.len());
        println!("{:?}", &ciphered_data);

        println!("Processed Object Size: {:?}", compressed_data.len());

        // Create the header with all the necessary information
        let mut object: Vec<u8> = vec!();
        object.extend(Compression::FOURCC.as_bytes());
        object.extend((ciphered_data.len() as u32).to_le_bytes());
        object.extend((compressed_data_pre_padding as u32).to_le_bytes());
        object.extend((data.len() as u32).to_le_bytes());
        object.extend(ciphered_data.to_owned());

        Ok(object)
    }

}


impl<Compression: Compressor, Ciphering: Cipher> EncryptedObject<Compression, Ciphering> {

    pub fn print_information(compression_magic: &[u8], cipher_magic: &[u8],
                             object_size: &usize, encrypted_size: &u32, compressed_size: &u32,
                             raw_size: &u32) {
        let cipher_fourcc = u32::from_le_bytes(cipher_magic.try_into().unwrap());

        println!("Compression Magic (FourCC): {:?}",
                 String::from_utf8(Vec::from(compression_magic)).unwrap());

        if cipher_fourcc == 0 {
            println!("Object is Not ciphered");
        } else {
            println!("Cipher Magic (FourCC): {:?}",
                     String::from_utf8_lossy(cipher_magic));
        }

        println!("Encrypted Size: {:?}", &encrypted_size);
        println!("Compressed Size: {:?}", &compressed_size);
        println!("Raw Size: {:?}", &raw_size);

        println!("Processed Object Size: {:?}", &object_size);
    }

}
