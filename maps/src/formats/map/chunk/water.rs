// Standard Uses

// Crate Uses

// External Uses
use nalgebra_glm::U8Vec2;


#[derive(Debug, PartialEq)]
pub struct Water {
    pub id: u16,
    pub width: u16,
    pub height: u16,
    pub enabled: bool,
    pub coordinates: Vec<(U8Vec2, u8)>
}

