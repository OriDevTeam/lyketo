// Standard Uses
use std::{fs::File, io::Read};
use byteorder::{ReadBytesExt, LittleEndian};
use nalgebra_glm::U8Vec2;

// Crate Uses

// External Uses
use crate::formats::map::chunk::{attributes::Attributes};


pub fn from_file(path: &str) -> Attributes {
    let file = File::options()
    .read(true)
    .open(path)
    .expect(stringify!("Cannot load chunk attribute file from {}", path));

    return from_bytes(file)
}

pub fn from_bytes(mut file: File) -> Attributes {
    let id = file.read_u16::<LittleEndian>().unwrap();
    let width = file.read_u16::<LittleEndian>().unwrap();
    let height = file.read_u16::<LittleEndian>().unwrap();

    let mut attributes = Attributes {
        id: id,
        width: width,
        height: height,
        coordinates: vec!()
    };
    
    let mut buf = Vec::new(); 
    let size = file.read_to_end(&mut buf).unwrap();

    for index in 0..size {
        let rect = U8Vec2::from([index as u8, (index / 2) as u8]);

        let flag = buf[index];

        attributes.coordinates.push((rect, flag));

    }

    return attributes
}
