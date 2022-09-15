// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use nalgebra_glm::{Vec2, Vec3};
use nalgebra_glm;


#[derive(Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal_position: Vec3,
    pub colors: Color,
    pub texture_position: Vec2,
    
}

impl Vertex {
    pub fn normalize(&mut self) {
        self.normal_position = nalgebra_glm::normalize(&self.normal_position);
    }
}
