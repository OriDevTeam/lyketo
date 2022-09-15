// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use nalgebra_glm::{Vec3, Vec2};
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct SkyBox {
    pub b_texture_render_mode: u8,
    pub scale: Vec3,
    pub gradient_level_upper: u8,
    pub gradient_level_lower: u8,

    pub front_face_file_name: String,
    pub back_face_file_name: String,
    pub left_face_file_name: String,
    pub top_face_file_name: String,
    pub bottom_face_file_name: String,
    
    pub cloud_scale: Vec2,
    pub cloud_height: f32,
    pub cloud_texture_scale: Vec2,
    pub cloud_speed: Vec2,
    pub cloud_texture_file_name: String,
    pub cloud_color: Vec<Color>,
    pub gradient: Vec<Color>
}

