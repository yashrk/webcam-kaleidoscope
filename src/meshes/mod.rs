use macroquad::models::Mesh;
mod hexagon;
mod triangles;

pub use hexagon::get_hex;
pub use triangles::get_triangles;

pub struct Figure {
    pub meshes: Vec<Vec<Mesh>>,
    pub mesh_ind: usize,
}

impl Figure {
    pub fn new(meshes: Vec<Vec<Mesh>>) -> Self {
        Figure {
            meshes,
            mesh_ind: 0,
        }
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        &self.meshes[self.mesh_ind]
    }
    pub fn next_mesh(&mut self) {
        self.mesh_ind = (self.mesh_ind + 1) % self.meshes.len();
    }
}
