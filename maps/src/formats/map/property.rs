// Standard Uses

// Crate Uses

// External Uses
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum MapType {
    Outdoor = 1
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Property {
    #[serde(alias="MapType")]
    pub map_type: MapType,
}

impl Property {
    pub(crate) fn new() -> Self { todo!() }
}

impl Property {}
