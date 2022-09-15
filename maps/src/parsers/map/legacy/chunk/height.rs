// Standard Uses
use std::{fs::File};

// External Uses
use crate::formats::map::chunk::{height::Height};

// External Uses
use byteorder::{ReadBytesExt, LittleEndian};
use nalgebra_glm::U8Vec2;


pub fn from_file(path: &str) -> Height {
    let file = File::options()
    .read(true)
    .open(path)
    .expect(stringify!("Cannot load chunk height file from {}", path));

    return from_bytes(file)
}

pub fn from_bytes(mut file: File) -> Height {
    let mut height = Height {
        coordinates: vec!()
    };

    let mut index = 0;
    for val in file.read_u16::<LittleEndian>() {
        let rect = U8Vec2::from([index as u8, (index / 2) as u8]);
        
        height.coordinates.push((rect, val));
        
        index += 1;
    }
    
    return height
}
