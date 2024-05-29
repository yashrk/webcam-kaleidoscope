use serde::Deserialize;

use crate::controls::Keyboard;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct WebCameraSettings {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Camera3DSettings {
    pub min_height: f32,
    pub max_height: f32,
    pub start_height: f32,
    pub step: f32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Trianges {
    pub levels: i16,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Hexagons {
    pub levels: i16,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Mesh {
    pub triangles: Trianges,
    pub hexagons: Hexagons,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Settings {
    pub keyboard: Keyboard,
    pub webcamera: WebCameraSettings,
    pub camera3d: Camera3DSettings,
    pub mesh: Mesh,
}
