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
        Some(KeyCode::Key8) => Some(Command::CameraReset),
        Some(KeyCode::Key2) => Some(Command::VShader("default".to_string())),
        Some(KeyCode::Key5) => Some(Command::FShader("default".to_string())),
        Some(KeyCode::L) => Some(Command::Shaders("crt".to_string(), "crt".to_string())),
        Some(KeyCode::C) => Some(Command::Shaders(
            "inner_rotation".to_string(),
            "test".to_string(),
        )),
        Some(KeyCode::B) => Some(Command::FShader("color".to_string())),
        Some(KeyCode::F) => Some(Command::FShader("glow".to_string())),
        Some(KeyCode::J) => Some(Command::FShader("rgb_corners".to_string())),
        Some(KeyCode::K) => Some(Command::FShader("corners".to_string())),
        Some(KeyCode::G) => Some(Command::FShader("time".to_string())),
        Some(KeyCode::I) => Some(Command::FShader("rainbow".to_string())),
        Some(KeyCode::A) => Some(Command::VShader("inner_rotation".to_string())),
        Some(KeyCode::E) => Some(Command::VShader("quake".to_string())),
        _ => None,
    }
}
