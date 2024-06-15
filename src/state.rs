use macroquad::models::Mesh;
use macroquad::prelude::*;

use crate::camera3d::CameraState;
use crate::material::{Shader, Style};
use crate::meshes::Figure;

pub struct State {
    pub style: Style,
    pub figure: Figure,
    pub camera: CameraState,
}

impl State {
    pub fn get_material(&self) -> Material {
        self.style.get_material()
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        self.figure.get_mesh()
    }
    pub fn increase_height(&mut self) {
        self.camera.increase_height();
    }
    pub fn decrease_height(&mut self) {
        self.camera.decrease_height();
    }
    pub fn reset_camera_heigth(&mut self) {
        self.camera.reset_heigth();
    }
    pub fn new(
        camera: CameraState,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
    ) -> Self {
        State {
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
            camera,
        }
    }
}
