use ffimage::color::Rgb;
use ffimage_yuv::yuv::Yuv;
use ffimage_yuv::yuv422::Yuyv;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

const WIDTH_U16: u16 = 960;
const HEIGHT_U16: u16 = 544;
const WIDTH_U32: u32 = 960;
const HEIGHT_U32: u32 = 544;
const BUFFER_COUNT: u32 = 4;

#[macroquad::main("Simple window")]
async fn main() {
    let mut dev = Device::new(0).expect("Failed to open device");
    let mut fmt = dev.format().expect("Failed to read format");

    fmt.width = WIDTH_U32;
    fmt.height = HEIGHT_U32;
    fmt.fourcc = FourCC::new(b"YVUV");
    let fmt = dev.set_format(&fmt).expect("Failed to write format");

    println!("Format in use:\n{}", fmt);

    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, BUFFER_COUNT)
        .expect("Failed to create buffer stream");
    let mut image = Image::gen_image_color(WIDTH_U16, HEIGHT_U16, WHITE);
    let texture = Texture2D::from_image(&image);

    let mut camera = Camera3D {
        position: vec3(-15., 15., -5.),
        up: vec3(0., 1., 0.),
        target: vec3(10., 5., 0.),
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
        set_camera(&camera);

        let image_data = image.get_image_data_mut();
        for i in 0..(buf.len() / 4) {
            let b1 = buf[i * 4];
            let b2 = buf[i * 4 + 1];
            let b3 = buf[i * 4 + 2];
            let b4 = buf[i * 4 + 3];
            let yuyv = Yuyv::<u8>::from([b1, b2, b3, b4]);
            let yuv = <[Yuv<u8>; 2]>::from(yuyv);
            let rgb1 = Rgb::<u8>::from(yuv[0]);
            let r = rgb1[0];
            let g = rgb1[1];
            let b = rgb1[2];
            let alpha = 255;
            let pixel = vec![r, g, b, alpha];
            image_data[i * 2] = pixel.try_into().unwrap();
            let rgb2 = Rgb::<u8>::from(yuv[0]);
            let r = rgb2[0];
            let g = rgb2[1];
            let b = rgb2[2];
            let alpha = 255;
            let pixel = vec![r, g, b, alpha];
            image_data[i * 2 + 1] = pixel.try_into().unwrap();
        }
        texture.update(&image);
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
            10. * Vec3::Y,
            10. * Vec3::X,
            10. * Vec3::Z,
            Some(&texture),
            WHITE,
        );
        next_frame().await
    }
}
