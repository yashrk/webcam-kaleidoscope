use macroquad::prelude::*;
use macroquad::Error;


pub fn get_material(vertex_shader: &String, fragment_shader: &String) -> Result<Material, Error> {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: fragment_shader,
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
