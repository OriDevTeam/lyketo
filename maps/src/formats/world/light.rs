// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use nalgebra_glm::Vec3;
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub background: Background,
    pub character: Character
}


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Background {
    pub enable: bool,
    pub diffuse: Color,
    pub ambient: Color,

}


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Character {
    pub enable: bool,
    pub diffuse: Color,
    pub ambient: Color
}
