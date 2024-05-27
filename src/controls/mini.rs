use macroquad::prelude::*;

use crate::controls::models::Command;

pub fn process_input() -> Option<Command> {
    match get_last_key_pressed() {
        Some(KeyCode::Escape) => Some(Command::Quit),
        Some(KeyCode::D) => Some(Command::SwitchRotation),
        Some(KeyCode::H) => Some(Command::NextMesh),
        Some(KeyCode::Key1) => Some(Command::NextVertexShader),
        Some(KeyCode::Key3) => Some(Command::PrevVertexShader),
        Some(KeyCode::Key6) => Some(Command::PrevFragmentShader),
        Some(KeyCode::Key4) => Some(Command::NextFragmentShader),
        Some(KeyCode::Key9) => Some(Command::CameraDown),
        Some(KeyCode::Key7) => Some(Command::CameraUp),
	Some(KeyCode::L) => Some(
	    Command::Shaders("crt".to_string(), "crt".to_string())),
	Some(KeyCode::C) => Some(
	    Command::Shaders("inner_rotation".to_string(), "test".to_string())),
        _ => None,
    }
}
