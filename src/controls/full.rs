use macroquad::prelude::*;

use crate::controls::models::Command;

const WHEEL_THRESHOLD: f32 = 0.01;
pub fn process_input() -> Option<Command> {
    match get_last_key_pressed() {
        Some(KeyCode::Escape) => Some(Command::Quit),
        Some(KeyCode::R) => Some(Command::SwitchRotation),
        Some(KeyCode::M) => Some(Command::NextMesh),
        Some(KeyCode::Up) => Some(Command::NextVertexShader),
        Some(KeyCode::Down) => Some(Command::PrevVertexShader),
        Some(KeyCode::Left) => Some(Command::PrevFragmentShader),
        Some(KeyCode::Right) => Some(Command::NextFragmentShader),
        Some(KeyCode::Key9) => Some(Command::DecreaseAngleSpeed),
        Some(KeyCode::Key0) => Some(Command::IncreaseAngleSpeed),
        Some(KeyCode::Key7) => Some(Command::DecreaseCycle),
        Some(KeyCode::Key8) => Some(Command::IncreaseCycle),
        _ => {
            if is_key_down(KeyCode::PageUp) {
                Some(Command::CameraUp)
            } else if is_key_down(KeyCode::PageDown) {
                Some(Command::CameraDown)
            } else {
                let (_, y) = mouse_wheel();
                if y > WHEEL_THRESHOLD {
                    Some(Command::CameraUp)
                } else if y < -WHEEL_THRESHOLD {
                    Some(Command::CameraDown)
                } else {
                    None
                }
            }
        }
    }
}
