// Standard Uses
use std::io::Cursor;
use std::mem;

// Crate Uses
use crate::utils::four_cc::FourCC;
use crate::utils::four_cc;

// External Uses
use bytemuck::{Zeroable, Pod};
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(C, align(4))]
pub struct Header {
    pub compression_magic: FourCC,
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
            compression_magic,
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
        if self.compressed_size > 0{
            if self.ciphered_size > 0 {
                return (
                    mem::size_of::<Self>() as u32 + mem::size_of::<FourCC>() as u32
                    + self.ciphered_size
                ) as u32
            }

            return (
                mem::size_of::<Self>() as u32 + mem::size_of::<FourCC>() as u32
                    + self.compressed_size
            ) as u32

        }

        self.raw_size
    }

    pub fn print_information(&self, object: &Vec<u8>, log: bool) {
        if !log {
            return
        };

        if self.ciphered_size > 0 && self.compressed_size > 0 {
            println!("Object is Ciphered and Compressed");
        } else if self.ciphered_size > 0 {
            println!("Object is Ciphered");
        } else if self.compressed_size > 0{
            println!("Object is Compressed");
        } else {
            println!("Object is Raw")
        }

        println!("Compression Magic (FourCC): {}",
                 four_cc::to_string(&self.compression_magic)
        );

        // TODO: Move this information display to ciphering stage
        /*
        if self.cipher_magic == 0 {
            println!("Object is Not ciphered");
        } else {
            println!("Cipher Magic (FourCC): {}", four_cc::to_string(&self.cipher_magic));
        }
        */

        println!("Encrypted Size: {}", &self.ciphered_size);
        println!("Compressed Size: {}", &self.compressed_size);
        println!("Raw Size: {}", &self.raw_size);

        println!("Processed Object Data Size: {:?}", &object.len());
    }

}

