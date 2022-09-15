/*
// Standard Uses
use std::fs::File;

// Crate Uses
use crate::formats::models::effect_version1::Effect;

// External Uses
use nom::bytes::complete::take_while_m_n;
use nom::{combinator::map_res};


pub fn parse_file(_path: &str) -> Effect {
  let file = File::options()
  .read(true)
  .open(_path)
  .expect(_path);

  return parse_bytes(file);
}

fn parse_bytes(mut file: File) -> Effect {
  /*
  let att = map_res(
    take_while_m_n(10, 10, is_hex_digit), from_hex
  )(file);
  */
  // att

  todo!()
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

/*
pub fn parse_bytes(mut file: File) -> Effect {

  let magic = "EffectData";

  let mut buf = [0u8; 10];
  file.read_exact(&mut buf).unwrap();

  assert_eq!(magic, buf);

  let effect = Effect {
      mesh_count: ,
      frame_count: 0,
      mesh_vector: vec!()
  };

  return effect
}
*/

*/
