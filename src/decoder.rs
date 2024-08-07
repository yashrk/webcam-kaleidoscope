use ffimage::color::Rgb;
use ffimage_yuv::yuv::Yuv;
use ffimage_yuv::yuv422::Yuyv;
use jpeg_decoder as jpeg;
use macroquad::prelude::*;

pub struct MjpegDecoder {}
pub struct YuyvDecoder {}

pub trait Decoder {
    fn decode(&self, image: &mut Image, buf: &[u8]);
}

impl Decoder for MjpegDecoder {
    fn decode(&self, image: &mut Image, buf: &[u8]) {
        let image_data = image.get_image_data_mut();
        let mut decoder = jpeg::Decoder::new(buf);
        let decoded = decoder.decode().expect("failed to decode JPEG");
        for i in 0..decoded.len() / 3 {
            let r = decoded[i * 3];
            let g = decoded[i * 3 + 1];
            let b = decoded[i * 3 + 2];
            let alpha = 255;
            let pixel = vec![r, g, b, alpha];
            image_data[i] = pixel.try_into().unwrap();
        }
    }
}
impl Decoder for YuyvDecoder {
    fn decode(&self, image: &mut Image, buf: &[u8]) {
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
            let rgb2 = Rgb::<u8>::from(yuv[1]);
            let r = rgb2[0];
            let g = rgb2[1];
            let b = rgb2[2];
            let alpha = 255;
            let pixel = vec![r, g, b, alpha];
            image_data[i * 2 + 1] = pixel.try_into().unwrap();
        }
    }
}
