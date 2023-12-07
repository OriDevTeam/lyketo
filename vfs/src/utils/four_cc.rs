// Standard Uses
use std::ffi::c_char;

// Crate Uses

// External Uses
use bytemuck;


/// Also called a magic, a 4 byte identifier used normally to identify
/// if a intended file format is what is being handled, it is located
/// usually on the start of the header of a file
pub type FourCC = u32;


pub fn from_chars(magic: [char; 4]) -> u32 {
    let word: [u8; 4] = magic.map(|c| c as u8);

    u32::from_le_bytes(word)
}

pub fn from_c_chars(magic: [c_char; 4]) -> u32 {
    let word = magic.map(|c| c as u8);

    u32::from_le_bytes(word)
}

pub fn from_bytes(bytes: [u8; 4]) -> FourCC {
    u32::from_le_bytes(bytes)
}


pub fn to_string(cc: FourCC) -> String {
    String::from_utf8_lossy(&cc.to_le_bytes()).to_string()
}


pub fn to_bytes(cc: u32) -> [u8; 4] {
    bytemuck::cast(cc)
}

