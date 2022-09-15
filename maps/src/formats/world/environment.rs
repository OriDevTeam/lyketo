// Standard Uses

// Crate Uses
use super::{
    light::DirectionalLight,
    material::Material,
    fog::Fog, skybox::SkyBox,
    filter::Filter,
    lens::LensFlare
};

// External Uses
use serde::{Serialize, Deserialize};



#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub directional_light: DirectionalLight,
    pub material: Material,
    pub fog: Fog,
    pub filter: Filter,
    pub sky_box: SkyBox,
    pub lens_flare: LensFlare   
}
