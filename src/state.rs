use macroquad::models::Mesh;
use macroquad::prelude::*;

use crate::material::{Shader, Style};
use crate::meshes::Figure;

pub struct State {
    pub camera_angle: f32,
    pub is_rotating: bool,
    pub style: Style,
    pub figure: Figure,
}

impl State {
    pub fn get_material(&self) -> Material {
        self.style.get_material()
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        self.figure.get_mesh()
    }
    pub fn new(
        camera_angle: f32,
        is_rotating: bool,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
    ) -> Self {
        State {
            camera_angle,
            is_rotating,
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
        }
    }
}
