use macroquad::prelude::*;

use crate::material::{get_material, Shader};

pub struct State {
    pub camera_angle: f32,
    pub is_rotating: bool,
    pub material: Material,
}

impl State {
    pub fn update_material(&mut self, vertex_shader: &Shader, fragment_shader: &Shader) {
        if let Ok(new_material) = get_material(vertex_shader, fragment_shader) {
            self.material = new_material;
        } else {
            println!("Unable to switch material, ignore")
        }
    }
}
