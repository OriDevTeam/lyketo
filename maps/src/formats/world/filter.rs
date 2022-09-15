// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub enable: bool,
    pub color: Color,
    pub alpha_src: u8,
    pub alpha_dest: u8,
}

