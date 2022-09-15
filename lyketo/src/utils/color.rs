// Standard Uses

// Crate Uses

// External Uses
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}
