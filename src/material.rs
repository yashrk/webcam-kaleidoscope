use macroquad::prelude::*;
use macroquad::Error;

#[derive(Debug, Clone)]
pub struct Shader {
    pub path: String,
    pub code: String,
}

impl std::fmt::Display for Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

pub fn get_material(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Material, Error> {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader.code,
            fragment: &fragment_shader.code,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                ("ms_time".to_owned(), UniformType::Float1),
                ("short_cycle".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
}
