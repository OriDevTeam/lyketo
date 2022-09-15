// Standard Uses

// Crate Uses

// External Uses


#[derive(Debug,PartialEq)]
pub struct Effect {
    pub mesh_count: u32,
    pub frame_count: u32,
    pub mesh_vector: Vec<Mesh>
}


#[derive(Debug,PartialEq)]
pub struct Mesh {
    pub name: String,
    pub diffuse_filename: String,
    pub frames: Vec<Frame>
}


#[derive(Debug,PartialEq)]
pub struct Frame {
    pub changed: bool,
    pub opacity: f32,
    pub vertex_count: u32,
    pub index_count: u32,
    pub texture_vertex_count: u32,
}

