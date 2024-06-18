use chrono::{DateTime, Utc};
use macroquad::models::Mesh;
use macroquad::prelude::*;

use crate::camera3d::CameraState;
use crate::material::{Shader, Style};
use crate::meshes::Figure;

pub struct State {
    pub style: Style,
    pub figure: Figure,
    pub camera: CameraState,
    pub cycle: u32,
    pub max_cycle: u32,
    pub default_cycle: u32,
    pub cycle_step: u32,
    pub phase: f32,
    pub dt: DateTime<Utc>,
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
    pub fn increase_cycle(&mut self) {
        self.cycle = u32::min(self.max_cycle, self.cycle + self.cycle_step);
    }
    pub fn decrease_cycle(&mut self) {
        self.cycle = u32::max(1, self.cycle - self.cycle_step);
    }
    pub fn reset_cycle(&mut self) {
        self.cycle = self.default_cycle;
    }
    pub fn next_phase(&mut self) {
        let dt = Utc::now();
        let td = (dt - self.dt).num_milliseconds();
        self.dt = dt;
        self.phase += (td as f32) / (self.cycle as f32);
        if self.phase > 1. {
            self.phase -= 1.;
        }
    }
    pub fn new(
        camera: CameraState,
        vertex_shaders: Vec<Shader>,
        fragment_shaders: Vec<Shader>,
        meshes: Vec<Vec<Mesh>>,
        default_cycle: u32,
        max_cycle: u32,
        cycle_step: u32,
    ) -> Self {
        State {
            style: Style::new(vertex_shaders, fragment_shaders),
            figure: Figure::new(meshes),
            camera,
            cycle: default_cycle,
            max_cycle,
            default_cycle,
            cycle_step,
            phase: 0.,
            dt: Utc::now(),
        }
    }
}
