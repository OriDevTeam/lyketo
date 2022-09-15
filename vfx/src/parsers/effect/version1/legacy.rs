// Standard Uses
use std::{fs::File, io::Read};

// Crate Uses
use crate::formats::effect::version1::{Effect, Mesh, Frame};

// External Uses
use byteorder::{ReadBytesExt, BigEndian};
use nalgebra_glm::{Vec3, Vec2};


pub fn parse_file(_path: &str) -> Effect {
  let file = File::options()
  .read(true)
  .open(_path)
  .expect(_path);

  return parse_bytes(file);
}

pub fn parse_bytes(mut file: File) -> Effect {

  let magic = "EffectData";

  let mut buf = [0u8; 10];
  file.read_exact(&mut buf).unwrap();

  assert_ne!(magic, stringify!(buf));

  let mesh_count= file.read_u16::<BigEndian>().unwrap();
  file.read_u16::<BigEndian>().unwrap();
  let frame_count =  file.read_u16::<BigEndian>().unwrap();
  file.read_u16::<BigEndian>().unwrap();

  let mut effect = Effect {
      mesh_count: mesh_count as u32,
      frame_count: frame_count as u32,
      meshes: vec!()
  };

  
  for _ in 0..effect.mesh_count {
    let mut name = [0u8; 32];
    file.read_exact(&mut name).unwrap();

    let mut diffuse_file_name = [0u8; 128];
    file.read_exact(&mut diffuse_file_name).unwrap();

    let vertex_count = file.read_u16::<BigEndian>().unwrap();
    file.read_u16::<BigEndian>().unwrap();

    let index_count = file.read_u16::<BigEndian>().unwrap();
    file.read_u16::<BigEndian>().unwrap();

    let texture_vertex_count = file.read_u16::<BigEndian>().unwrap();
    file.read_u16::<BigEndian>().unwrap();

    let mut mesh = Mesh {
        name: stringify!(name).to_string(),
        diffuse_filename: stringify!(diffuse_file_name).to_string(),
        vertex_count: vertex_count as u32,
        index_count: index_count as u32,
        texture_vertex_count: texture_vertex_count as u32,
        frames: vec!(),
    };

    for _ in 0..effect.frame_count {
      let mut frame = Frame {
        opacity: file.read_f32::<BigEndian>().unwrap(),
        vertices: vec!(),
        faces: vec!(),
        texture_vertices: vec!(),
        texture_faces: vec!(),
      };

      for _ in 0..mesh.vertex_count {
        let axis = Vec3::new(
            file.read_f32::<BigEndian>().unwrap(),
            file.read_f32::<BigEndian>().unwrap(),
            file.read_f32::<BigEndian>().unwrap(),
        );

        frame.vertices.push(axis);
      }
      
      for _ in 0..mesh.index_count {
          let index = file.read_u32::<BigEndian>().unwrap();
          frame.faces.push(index);
      }

      for _ in 0..mesh.texture_vertex_count {
        let axis = Vec2::new(
            file.read_f32::<BigEndian>().unwrap(),
            file.read_f32::<BigEndian>().unwrap(),
        );

        frame.texture_vertices.push(axis);
      }

      for _ in 0..mesh.index_count {
        let index = file.read_u32::<BigEndian>().unwrap();
        frame.texture_faces.push(index);
    }

      mesh.frames.push(frame);
    }

    effect.meshes.push(mesh);

  }

  return effect
}
