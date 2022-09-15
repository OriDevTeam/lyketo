// Standard Uses
use std::{fs::File, io::Read};
use byteorder::{ReadBytesExt, LittleEndian};
use nalgebra_glm::U8Vec2;

// External Uses
use crate::formats::map::chunk::{water::Water};


pub fn from_file(path: &str) -> Water {
    let file = File::options()
    .read(true)
    .open(path)
    .expect(stringify!("Cannot load chunk water file from {}", path));

    return from_bytes(file)
}

pub fn from_bytes(mut file: File) -> Water {
    let id = file.read_u16::<LittleEndian>().unwrap();
    let width = file.read_u16::<LittleEndian>().unwrap();
    let height = file.read_u16::<LittleEndian>().unwrap();
    let enabled = file.read_u8().unwrap();

    let mut water = Water {
        id: id,
        width: width,
        height: height,
        enabled: enabled != 0,
        coordinates: vec!()
    };
    
    let mut buf = Vec::new(); 
    let size = file.read_to_end(&mut buf).unwrap();

    for index in 0..size {
        let rect = U8Vec2::from([index as u8, (index / 2) as u8]);
        let flag = buf[index];

        water.coordinates.push((rect, flag));

    }

    return water
}
