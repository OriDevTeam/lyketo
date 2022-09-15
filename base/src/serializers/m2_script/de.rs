// Standard Uses

// Crate Uses
use crate::serializers::m2_script::error::Error;
use crate::serializers::m2_script::error::Result;

// External Uses
use serde::Deserialize;
use serde::de::{
    DeserializeSeed
};

/*
pub struct Deserializer<'de> {

    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}
*/

