use macroquad::models::Mesh;
use macroquad::prelude::*;

use crate::material::{Shader, Style};
use crate::meshes::Figure;

pub struct State {
    pub camera_angle: f32,
    pub camera_height: f32,
    pub is_rotating: bool,
    pub style: Style,
    pub figure: Figure,
}

const HEIGHT_STEP: f32 = 0.05;
const MIN_HEIGHT: f32 = -7.0;
const MAX_HEIGHT: f32 = 7.0;

impl State {
    pub fn get_material(&self) -> Material {
        self.style.get_material()
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        self.figure.get_mesh()
    }
    pub fn increase_height(&mut self) {
        self.camera_height = f32::min(MAX_HEIGHT, self.camera_height + HEIGHT_STEP);
    }
    pub fn decrease_height(&mut self) {
        self.camera_height = f32::max(MIN_HEIGHT, self.camera_height - HEIGHT_STEP);
    }
    pub fn new(
        camera_angle: f32,
        camera_height: f32,
        is_rotating: bool,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
    ) -> Self {
        State {
            camera_angle,
            camera_height,
            is_rotating,
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
        }
    }
}
