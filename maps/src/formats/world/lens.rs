// Standard Uses

// Crate Uses

// External Uses
use lyketo::utils::color::Color;

use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct LensFlare {
    pub enable: bool,
    pub brightness_color: Color,
    pub max_brightness: f32,
    pub main_flare_enable: bool,
    pub main_flare_texture_file_name: String,
    pub main_flare_size: f32
}
