use chrono::{Timelike, Utc};
use config::{Config, File};
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

use webcam::config::Settings;
use webcam::controls::{full_process_input, mini_process_input, Command, Keyboard};
use webcam::decoder::*;
use webcam::material::Shader;
use webcam::meshes::{get_hexagons, get_triangles};
use webcam::state::State;

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
    let settings = Config::builder()
        .add_source(File::with_name("config/default.json").required(false))
        .add_source(File::with_name("config/local.json").required(false))
        .build()
        .unwrap();
    // Deserialize the config object into your Settings struct:
    let settings: Settings = settings.try_deserialize().unwrap();

    let dev = Device::new(0).expect("Failed to open device");
    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = settings.webcamera.width;
    fmt.height = settings.webcamera.height;
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

    let mut state = State::new(
        settings.camera3d.start_height,
        settings.camera3d.min_height,
        settings.camera3d.max_height,
        settings.camera3d.step,
        vertex_shaders,
        fragment_shaders,
        vec![
            get_triangles(settings.mesh.triangles.levels, texture.clone()),
            get_hexagons(settings.mesh.hexagons.levels, texture.clone()),
        ],
    );
    state
        .style
        .set_shaders("default".to_string(), "default".to_string());
    let mut camera = Camera3D {
        position: vec3(0., 0., state.camera_height),
        up: angle2vec(state.camera_angle),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let process_input = match settings.keyboard {
        Keyboard::Full => full_process_input,
        Keyboard::Mini => mini_process_input,
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
        match process_input() {
            Some(Command::Quit) => break,
            Some(Command::SwitchRotation) => state.is_rotating = !state.is_rotating,
            Some(Command::NextMesh) => {
                state.figure.next_mesh();
            }
            Some(Command::NextVertexShader) => {
                state.style.next_vertex_shader();
            }
            Some(Command::PrevVertexShader) => {
                state.style.prev_vertex_shader();
            }
            Some(Command::PrevFragmentShader) => {
                state.style.prev_fragment_shader();
            }
            Some(Command::NextFragmentShader) => {
                state.style.next_fragment_shader();
            }
            Some(Command::CameraDown) => {
                state.decrease_height();
            }
            Some(Command::CameraUp) => {
                state.increase_height();
            }
            Some(Command::CameraReset) => {
                state.reset_camera_heigth();
            }
            Some(Command::Shaders(vshader, fshader)) => {
                state.style.set_shaders(vshader, fshader);
            }
            Some(Command::FShader(fshader)) => {
                state.style.set_fragment_shader(fshader);
            }
            Some(Command::VShader(vshader)) => {
                state.style.set_vetrex_shader(vshader);
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
        let material = state.get_material();
        material.set_uniform("ms_time", millis_since_midnight as f32);
        material.set_uniform("short_cycle", short_cycle as f32);
        if state.is_rotating {
            state.camera_angle += 0.01;
        }
        if state.camera_angle > 2.0 * PI {
            state.camera_angle = 0.;
        }

        camera.up = angle2vec(state.camera_angle);
        camera.position = vec3(0., 0., state.camera_height);
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
        gl_use_material(&state.get_material());
        state.get_mesh().iter().for_each(draw_mesh);
        gl_use_default_material();

        next_frame().await
    }
}
