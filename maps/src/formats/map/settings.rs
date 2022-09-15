// Standard Uses

// Crate Uses
// use crate::utils::algebra::U8Vec2Ref;

// External Uses
use nalgebra_glm::U8Vec2;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(alias="CellScale")]
    pub cell_scale: f32,

    #[serde(alias="HeightScale")]
    pub height_scale: f32,

    #[serde(alias="ViewRadius")]
    pub view_radius: f32,

    #[serde(alias="MapSize")]
    // #[serde(with = "U8Vec2Ref")]
    pub map_size: U8Vec2,

    #[serde(alias="BasePosition")]
    // #[serde(with = "U8Vec2Ref")]
    pub base_position: U8Vec2,

    #[serde(alias="TextureSet")]
    pub texture_set: String,

    #[serde(alias="Environment")]
    pub environment: String
}

impl Settings {
    pub fn new() -> Self { todo!() }
}


/*
Settings {
                CellScale: 100.0,
                height_scale: 0.5,
                ViewRadius: 100.0,
                MapSize: Rectangle { x: 1, y: 1 },
                BasePosition: Rectangle { x: 0, y: 0 },
                TextureSet: "".to_string(),
                Environment: "".to_string(),
            },
*/

