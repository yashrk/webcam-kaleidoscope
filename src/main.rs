use macroquad::prelude::*;
use macroquad::texture::Texture2D;
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

#[macroquad::main("Simple window")]
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

    let camera = Camera3D {
        position: vec3(-15., 15., -5.),
        up: vec3(0., 0.1, 0.),
        target: vec3(5., 0., 0.),
        ..Default::default()
    };

    loop {
        //
        // Webcam
        //
        let (buf, _meta) = stream.next().unwrap();
        //
        // GUI
        //
        decode(&mut image, buf);
        texture.update(&image);

        clear_background(LIGHTGRAY);

        set_camera(&camera);

        // draw_texture(&texture, 0., 0., WHITE);
        draw_grid(1, 0.1, WHITE, RED);
        draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), Some(&texture), WHITE);

        draw_affine_parallelogram(
            Vec3::ZERO,
            10. * Vec3::X,
            10. * Vec3::Z,
            Some(&texture),
            WHITE,
        );
        draw_affine_parallelogram(
            10. * Vec3::NEG_Y,
            10. * Vec3::X,
            10. * Vec3::Y,
            Some(&texture),
            WHITE,
        );
        draw_affine_parallelogram(
            10. * Vec3::NEG_Y,
            10. * Vec3::X,
            10. * Vec3::NEG_Z,
            Some(&texture),
            WHITE,
        );
        draw_affine_parallelogram(
            10. * Vec3::X,
            10. * Vec3::X,
            10. * Vec3::NEG_Z,
            Some(&texture),
            WHITE,
        );
        draw_affine_parallelogram(
            vec3(10., 0., 0.),
            10. * Vec3::NEG_Y,
            10. * Vec3::NEG_Z,
            Some(&texture),
            WHITE,
        );
        draw_affine_parallelogram(
            vec3(5.0, 0., 0.),
            10. * Vec3::NEG_X,
            10. * Vec3::Z,
            Some(&texture),
            WHITE,
        );
        set_default_camera();
        next_frame().await
    }
}
