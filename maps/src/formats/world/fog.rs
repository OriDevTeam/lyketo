// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Fog {
    pub enable: bool,
    pub near_distance: f32,
    pub far_distance: f32,
    pub color: Color,
}


