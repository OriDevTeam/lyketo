// Standard Uses

// Crate Uses
use super::{
    segment::{Segment},
    chunk::Chunk
};


// External Uses


pub struct Mesh {
    _indices: Vec<u16>,
    _vertices: Vec<Segment>,
}

impl Mesh {

    pub fn generate(chunk: &Chunk) -> Mesh {
        let mut mesh: Mesh = Mesh {
            _indices: Vec::new(),
            _vertices: Vec::new()
        };

        //mesh.generate_splat_maps(&chunk);
        mesh.generate_mesh_data(&chunk);
        mesh.calculate_normals(&chunk);

        return mesh;
    }

    fn generate_mesh_data(& mut self, chunk: &Chunk) {
        let size = chunk.size();

        for y in 1..size {
            for x in 1..size {
                
                let _height = chunk.height_at(x, y);
                let _index = index(size, y, x);
                
                /*
                self.vertices[index].positions = Vec3::new(
                    x.into(),
                    y.into(),
                    (chunk.size() - x).into()
                );
                 */
                
                let _x: f32 = (x / size).into();
                let _y: f32 = (y / size).into();
                
                /*
                self.vertices[index].texture_coordinates = TextureCoordinates {
                    x: min_partial!(0.0, max_partial!(_x, 0.99)),
                    y: min_partial!(0.0, max_partial!(_y, 0.99))
                };
                */

                // TODO: Fill indices vector
            }
        };
    }

    fn calculate_normals(&self, _chunk: &Chunk) {

    }
    
}

fn index(size: u16, x: u16, y: u16) -> usize {
    return (x + y * (size + 1)).into()
}
