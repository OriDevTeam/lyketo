// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Material {
    pub diffuse: Color,
    pub ambient: Color,
    pub emissive: Color,
}
