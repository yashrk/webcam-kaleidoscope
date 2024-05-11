use glob::glob;
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::fs;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

use webcam::decoder::*;

const WIDTH_U32: u32 = 640;
const HEIGHT_U32: u32 = 480;
const BUFFER_COUNT: u32 = 4;

struct State {
    camera_angle: f32,
    is_rotating: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Kaleidoscope".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn angle2vec(angle: f32) -> Vec3 {
    let x = angle.sin();
    let y = angle.cos();
    vec3(x, y, 0.)
}

#[macroquad::main(window_conf)]
async fn main() {
    let dev = Device::new(0).expect("Failed to open device");
    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = WIDTH_U32;
    fmt.height = HEIGHT_U32;
    fmt.fourcc = FourCC::new(b"YVUV");
    println!("Format to set:\n{}", fmt);
    let fmt = dev.set_format(&fmt).expect("Failed to write format");
    println!("Format in use:\n{}", fmt);
    let decode = if fmt.fourcc == FourCC::new(b"MJPG") {
        decode_mjpeg
    } else {
        decode_yuyv
    };
    let mut stream = Stream::with_buffers(&dev, Type::VideoCapture, BUFFER_COUNT)
        .expect("Failed to create buffer stream");
    let mut image = Image::gen_image_color(fmt.width as u16, fmt.height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    let fragment_shaders: Vec<_> = glob("resources/*.fs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            fs::read_to_string(x.to_str().unwrap()).unwrap()
        })
        .collect();
    let vertex_shaders: Vec<_> = glob("resources/*.vs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            fs::read_to_string(x.to_str().unwrap()).unwrap()
        })
        .collect();

    let mut v_shader_ind: usize = 0;
    let mut f_shader_ind: usize = 0;

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    let mut material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shaders[v_shader_ind],
            fragment: &fragment_shaders[f_shader_ind],
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    let mut state = State {
        camera_angle: 0.,
        is_rotating: true,
    };
    let mut camera = Camera3D {
        position: vec3(0., 0., -5.),
        up: angle2vec(state.camera_angle),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let mesh_1 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(0., 2., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(2., 1., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };
    let mesh_2 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(-2., 1., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(0., 2., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };
    let mesh_3 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(-2., -1., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(-2., 1., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };
    let mesh_4 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(-2., -1., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(0., -2., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };
    let mesh_5 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(0., -2., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(2., -1., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };
    let mesh_6 = Mesh {
        vertices: vec![
            Vertex {
                position: vec3(0., 0., 0.),
                uv: vec2(0., 0.),
                color: WHITE,
            },
            Vertex {
                position: vec3(2., 1., 0.),
                uv: vec2(0., 1.),
                color: WHITE,
            },
            Vertex {
                position: vec3(2., -1., 0.),
                uv: vec2(1., 1.),
                color: WHITE,
            },
        ],
        indices: vec![0, 1, 2],
        texture: Some(texture.clone()),
    };

    loop {
        //
        // Webcam
        //
        let (buf, _meta) = stream.next().unwrap();
        decode(&mut image, buf);

        //
        // Input
        //
        match get_last_key_pressed() {
            Some(KeyCode::Escape) => break,
            Some(KeyCode::R) => state.is_rotating = !state.is_rotating,
            Some(KeyCode::Up) => {
                v_shader_ind = (v_shader_ind + 1) % vertex_shaders.len();
                material = load_material(
                    ShaderSource::Glsl {
                        vertex: &vertex_shaders[v_shader_ind],
                        fragment: &fragment_shaders[f_shader_ind],
                    },
                    MaterialParams {
                        pipeline_params,
                        ..Default::default()
                    },
                )
                .unwrap();
                println!("Vertex shader {}", v_shader_ind)
            }
            Some(KeyCode::Down) => {
                v_shader_ind =
                    (v_shader_ind as i16 - 1).rem_euclid(vertex_shaders.len() as i16) as usize;
                material = load_material(
                    ShaderSource::Glsl {
                        vertex: &vertex_shaders[v_shader_ind],
                        fragment: &fragment_shaders[f_shader_ind],
                    },
                    MaterialParams {
                        pipeline_params,
                        ..Default::default()
                    },
                )
                .unwrap();
                println!("Vertex shader {}", v_shader_ind)
            }
            Some(KeyCode::Left) => {
                f_shader_ind =
                    (f_shader_ind as i16 - 1).rem_euclid(fragment_shaders.len() as i16) as usize;
                material = load_material(
                    ShaderSource::Glsl {
                        vertex: &vertex_shaders[v_shader_ind],
                        fragment: &fragment_shaders[f_shader_ind],
                    },
                    MaterialParams {
                        pipeline_params,
                        ..Default::default()
                    },
                )
                .unwrap();
                println!("Fragment shader {}", f_shader_ind)
            }
            Some(KeyCode::Right) => {
                f_shader_ind = (f_shader_ind + 1) % fragment_shaders.len();
                material = load_material(
                    ShaderSource::Glsl {
                        vertex: &vertex_shaders[v_shader_ind],
                        fragment: &fragment_shaders[f_shader_ind],
                    },
                    MaterialParams {
                        pipeline_params,
                        ..Default::default()
                    },
                )
                .unwrap();
                println!("Fragment shader {}", f_shader_ind)
            }
            _ => (),
        }

        //
        // GUI
        //
        texture.update(&image);
        if state.is_rotating {
            state.camera_angle += 0.01;
        }
        if state.camera_angle > 2.0 * 3.14 {
            state.camera_angle = 0.;
        }

        camera.up = angle2vec(state.camera_angle);
        clear_background(BLACK);
        set_camera(&camera);
        draw_grid_ex(
            20,
            0.1,
            RED,
            GRAY,
            Vec3::ZERO,
            Quat::from_xyzw(0., 1., 1., 0.),
        );
        gl_use_material(&material);
        draw_mesh(&mesh_1);
        draw_mesh(&mesh_2);
        draw_mesh(&mesh_3);
        draw_mesh(&mesh_4);
        draw_mesh(&mesh_5);
        draw_mesh(&mesh_6);
        gl_use_default_material();

        next_frame().await
    }
}
