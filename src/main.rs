use chrono::{Timelike, Utc};
use glob::glob;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::f32::consts::PI;
use std::fs;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

use webcam::decoder::*;
use webcam::material::{get_material, Shader};
use webcam::meshes::{get_hex, get_triangles};
use webcam::state::State;

const WIDTH_U32: u32 = 640;
const HEIGHT_U32: u32 = 480;
const BUFFER_COUNT: u32 = 4;

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

    let fragment_shaders: Vec<_> = glob("shaders/*.fs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            Shader {
                path: x.to_str().unwrap().to_string(),
                code: fs::read_to_string(x.to_str().unwrap()).unwrap(),
            }
        })
        .collect();
    let vertex_shaders: Vec<_> = glob("shaders/*.vs")
        .expect("Failed to read glob pattern")
        .flatten()
        .map(|x| {
            println!("{:?}", x.display());
            Shader {
                path: x.to_str().unwrap().to_string(),
                code: fs::read_to_string(x.to_str().unwrap()).unwrap(),
            }
        })
        .collect();

    let mut v_shader_ind: usize = 0;
    let mut f_shader_ind: usize = 0;

    let mut state = State {
        camera_angle: 0.,
        is_rotating: true,
        material: get_material(
            &vertex_shaders[v_shader_ind],
            &fragment_shaders[f_shader_ind],
        )
        .unwrap(),
    };
    let mut camera = Camera3D {
        position: vec3(0., 0., -5.),
        up: angle2vec(state.camera_angle),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let meshes = vec![get_triangles(3, texture.clone()), get_hex(texture.clone())];
    let mut mesh_ind = 0;
    let mut mesh = &meshes[mesh_ind as usize];
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
            Some(KeyCode::M) => {
                mesh_ind = (mesh_ind + 1) % (meshes.len() as i32);
                mesh = &meshes[mesh_ind as usize];
            }
            Some(KeyCode::Up) => {
                v_shader_ind = (v_shader_ind + 1) % vertex_shaders.len();
                println!("Vertex shader {}", vertex_shaders[v_shader_ind]);
                state.update_material(
                    &vertex_shaders[v_shader_ind],
                    &fragment_shaders[f_shader_ind],
                );
            }
            Some(KeyCode::Down) => {
                v_shader_ind =
                    (v_shader_ind as i16 - 1).rem_euclid(vertex_shaders.len() as i16) as usize;
                println!("Vertex shader {}", vertex_shaders[v_shader_ind]);
                state.update_material(
                    &vertex_shaders[v_shader_ind],
                    &fragment_shaders[f_shader_ind],
                );
            }
            Some(KeyCode::Left) => {
                f_shader_ind =
                    (f_shader_ind as i16 - 1).rem_euclid(fragment_shaders.len() as i16) as usize;
                println!("Fragment shader {}", fragment_shaders[f_shader_ind]);
                state.update_material(
                    &vertex_shaders[v_shader_ind],
                    &fragment_shaders[f_shader_ind],
                );
            }
            Some(KeyCode::Right) => {
                f_shader_ind = (f_shader_ind + 1) % fragment_shaders.len();
                println!("Fragment shader {}", fragment_shaders[f_shader_ind]);
                state.update_material(
                    &vertex_shaders[v_shader_ind],
                    &fragment_shaders[f_shader_ind],
                );
            }
            _ => (),
        }

        //
        // GUI
        //
        texture.update(&image);
        // Current time (since last midnight, in milliseconds)
        let now = Utc::now();
        let millis_since_midnight =
            (now.num_seconds_from_midnight() * 1000) + now.timestamp_subsec_millis();
        // We want to leave only 5 last digits
        let short_cycle = millis_since_midnight % 30000;
        state
            .material
            .set_uniform("ms_time", millis_since_midnight as f32);
        state
            .material
            .set_uniform("short_cycle", short_cycle as f32);
        if state.is_rotating {
            state.camera_angle += 0.01;
        }
        if state.camera_angle > 2.0 * PI {
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
            vec3(0.0, 0.0, 0.5),
            Quat::from_xyzw(0., 1., 1., 0.),
        );
        gl_use_material(&state.material);
        mesh.iter().for_each(draw_mesh);
        gl_use_default_material();

        next_frame().await
    }
}
