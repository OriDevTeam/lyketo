
// Standard Uses


// Crate Uses

// External Uses
use bitflags::bitflags;
use nalgebra_glm::U8Vec2;


bitflags! {
    pub struct AttributeFlags: u8 {
        const WALK = 0b00000001;
        const WATER = 0b0000001;
        const BATTLE = 0b00000010;
        const BUILD = 0b00000100;
        const ALL = Self::WALK.bits | Self::WATER.bits | Self::BATTLE.bits | Self::BUILD.bits;
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub id: u16,
    pub width: u16,
    pub height: u16,
    pub coordinates: Vec<(U8Vec2, u8)>
}

