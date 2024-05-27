use serde::Deserialize;

use crate::controls::Keyboard;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct WebCameraSettings {
    pub width: u32,
    pub height: u32
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub keyboard: Keyboard,
    pub webcamera: WebCameraSettings,
}
