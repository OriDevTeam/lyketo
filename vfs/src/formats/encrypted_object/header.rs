// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc::FourCC;
use crate::utils::four_cc;

// External Uses
use eyre::Result;
use bytemuck::{Zeroable, Pod};
use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Copy, Clone)]
#[derive(Zeroable, Pod)]
#[repr(C, align(4))]
pub struct Header {
    pub ciphering_magic: FourCC,
    pub ciphered_size: u32,
    pub compressed_size: u32,
    pub raw_size: u32,
}


#[allow(unused)]
impl Header {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let compression_magic = cursor.read_u32::<LittleEndian>()?;
        let ciphered_size = cursor.read_u32::<LittleEndian>()?;
        let compressed_size = cursor.read_u32::<LittleEndian>()?;
        let raw_size = cursor.read_u32::<LittleEndian>()?;

        Ok(Self {
            ciphering_magic: compression_magic,
            ciphered_size,
            compressed_size,
            raw_size,
        })
    }

    // TODO; Check how to do this cast, since it reduces the code on 'from_bytes' above
    /*
    pub fn from_bytes_cast(data: Vec<u8>) -> Result<Self> {
        if data.len() < mem::size_of::<Header>() {
            bail!("Expected exactly {} bytes, got {} instead",
                mem::size_of::<Header>(), &data.len()
            )
        }

        bytemuck::try_cast::<_, Header>(&data[..mem::size_of::<Header>()])
            .map_err(|err| anyhow!("{}", err))
    }
    */

    pub fn object_size(&self) -> u32 {
        if self.compressed_size > 0 {
            if self.ciphered_size > 0 {
                return std::mem::size_of::<Self>() as u32 + std::mem::size_of::<FourCC>() as u32
                    + self.ciphered_size
            }

            return std::mem::size_of::<Self>() as u32 + std::mem::size_of::<FourCC>() as u32
                + self.compressed_size
        }

        self.raw_size
    }

    pub fn print_pre_deciphering_info(&self) {
        if self.ciphered_size > 0 && self.compressed_size > 0 {
            println!("Object is Ciphered and Compressed (according to Header)");
        } else if self.ciphered_size > 0 {
            println!("Object is Ciphered only (according to Header)");
        } else if self.compressed_size > 0{
            println!("Object is Compressed only (according to Header)");
        } else {
            println!("Object is Raw")
        }

        println!(
            "Compression Magic (FourCC): {}", four_cc::to_string(self.ciphering_magic)
        );

        println!("Encrypted Size (in Header): {}", &self.ciphered_size);
        println!("Compressed Size (in Header): {}", &self.compressed_size);
        println!("Raw Size (in Header): {}", &self.raw_size);
    }

    pub fn print_pre_decompression_info(&self) {
        // TODO: This only reflects what the header says, actual data
        //       might tell something different and we should rely on data
        /*
        if self.ciphered_size == 0 {
            println!("Object is Not ciphered, skipping decipher");
        } else {
            println!("Object Is ciphered")
            // println!("Cipher Magic (FourCC): {}", four_cc::to_string(&self.cipher_magic));
        }
        */
    }
}

