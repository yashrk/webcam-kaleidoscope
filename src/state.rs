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
    pub min_camera_height: f32,
    pub max_camera_height: f32,
    pub start_camera_height: f32,
    pub camera_step: f32,
}

impl State {
    pub fn get_material(&self) -> Material {
        self.style.get_material()
    }
    pub fn get_mesh(&self) -> &Vec<Mesh> {
        self.figure.get_mesh()
    }
    pub fn increase_height(&mut self) {
        self.camera_height = f32::min(
            self.max_camera_height,
            self.camera_height + self.camera_step,
        );
    }
    pub fn decrease_height(&mut self) {
        self.camera_height = f32::max(
            self.min_camera_height,
            self.camera_height - self.camera_step,
        );
    }
    pub fn reset_camera_heigth(&mut self) {
        self.camera_height = self.start_camera_height;
    }
    pub fn new(
        camera_angle: f32,
        camera_height: f32,
        min_camera_height: f32,
        max_camera_height: f32,
        camera_step: f32,
        is_rotating: bool,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
    ) -> Self {
        State {
            camera_angle,
            camera_height,
            min_camera_height,
            max_camera_height,
            camera_step,
            is_rotating,
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
            start_camera_height: camera_height,
        }
    }
}
