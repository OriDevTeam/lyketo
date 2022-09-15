// Standard Uses

// Crate Uses

// External Uses
use nalgebra_glm::{Vec3, Vec2};


#[derive(Debug,PartialEq)]
pub struct Effect {
    pub mesh_count: u32,
    pub frame_count: u32,
    pub meshes: Vec<Mesh>
}


#[derive(Debug,PartialEq)]
pub struct Mesh {
    pub name: String,
    pub diffuse_filename: String,
    pub vertex_count: u32,
    pub texture_vertex_count: u32,
    pub index_count: u32,
    pub frames: Vec<Frame>
}


#[derive(Debug,PartialEq)]
pub struct Frame {
    pub opacity: f32,
    pub vertices: Vec<Vec3>,
    pub faces: Vec<u32>,
    pub texture_vertices: Vec<Vec2>,
    pub texture_faces: Vec<u32>
}

