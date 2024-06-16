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
#[derive(Debug, Clone)]
pub struct Style {
    pub vertex_shaders: Vec<Shader>,
    pub fragment_shaders: Vec<Shader>,
    pub v_shader_ind: usize,
    pub f_shader_ind: usize,
    pub material: Material,
}

impl Style {
    pub fn new(vertex_shaders: Vec<Shader>, fragment_shaders: Vec<Shader>) -> Self {
        let material = get_material(&vertex_shaders[0], &fragment_shaders[0]).unwrap();
        Style {
            vertex_shaders,
            fragment_shaders,
            v_shader_ind: 0,
            f_shader_ind: 0,
            material,
        }
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }

    pub fn set_material(&mut self) {
        match get_material(
            &self.vertex_shaders[self.v_shader_ind],
            &self.fragment_shaders[self.f_shader_ind],
        ) {
            Ok(new_material) => {
                self.material = new_material;
            }
            Err(Error::ShaderError(e)) => {
                println!(
                    "Unable to switch material to {} and {}: {}, ignore",
                    self.vertex_shaders[self.v_shader_ind],
                    self.fragment_shaders[self.f_shader_ind],
                    e
                );
            }
            _ => {
                println!(
                    "Unable to switch material to {} and {}: unknown error, ignore",
                    self.vertex_shaders[self.v_shader_ind],
                    self.fragment_shaders[self.f_shader_ind],
                );
            }
        }
    }

    pub fn set_vetrex_shader(&mut self, name: String) {
        if let Some(ind) = self
            .vertex_shaders
            .iter()
            .position(|x| x.path == format!("shaders/{}.vs", name))
        {
            self.v_shader_ind = ind;
            println!("Setting vertex shader to {name} by name");
            self.set_material()
        }
    }
    pub fn set_fragment_shader(&mut self, name: String) {
        if let Some(ind) = self
            .fragment_shaders
            .iter()
            .position(|x| x.path == format!("shaders/{}.fs", name))
        {
            self.f_shader_ind = ind;
            println!("Setting fragment shader to {name} by name");
            self.set_material()
        }
    }

    pub fn set_shaders(&mut self, vshader: String, fshader: String) {
        if let Some(ind) = self
            .vertex_shaders
            .iter()
            .position(|x| x.path == format!("shaders/{}.vs", vshader))
        {
            self.v_shader_ind = ind;
        }
        if let Some(ind) = self
            .fragment_shaders
            .iter()
            .position(|x| x.path == format!("shaders/{}.fs", fshader))
        {
            self.f_shader_ind = ind;
        }
        println!("Setting fragment shader to {fshader}, vertex shader to {vshader} by name");
        self.set_material()
    }

    pub fn next_vertex_shader(&mut self) {
        self.v_shader_ind = (self.v_shader_ind + 1) % self.vertex_shaders.len();
        println!("Vertex shader {}", self.vertex_shaders[self.v_shader_ind]);
        self.set_material()
    }
    pub fn prev_vertex_shader(&mut self) {
        self.v_shader_ind =
            (self.v_shader_ind + self.vertex_shaders.len() - 1) % self.vertex_shaders.len();
        println!("Vertex shader {}", self.vertex_shaders[self.v_shader_ind]);
        self.set_material()
    }
    pub fn next_fragment_shader(&mut self) {
        self.f_shader_ind = (self.f_shader_ind + 1) % self.fragment_shaders.len();
        println!(
            "Fragment shader {}",
            self.fragment_shaders[self.f_shader_ind]
        );
        self.set_material()
    }
    pub fn prev_fragment_shader(&mut self) {
        self.f_shader_ind =
            (self.f_shader_ind + self.fragment_shaders.len() - 1) % self.fragment_shaders.len();
        println!(
            "Fragment shader {}",
            self.fragment_shaders[self.f_shader_ind]
        );
        self.set_material()
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
                ("phase".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
}
