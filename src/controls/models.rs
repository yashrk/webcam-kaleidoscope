use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Default, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Keyboard {
    #[default]
    Full,
    Mini,
}

pub enum Command {
    Quit,
    NextMesh,
    CameraUp,
    CameraDown,
    CameraReset,
    SwitchRotation,
    NextFragmentShader,
    PrevFragmentShader,
    NextVertexShader,
    PrevVertexShader,
    FShader(String),
    VShader(String),
    Shaders(String, String),
    IncreaseAngleSpeed,
    DecreaseAngleSpeed,
}
