#![cfg_attr(windows, feature(abi_vectorcall))]

use std::io::BufWriter;
use ext_php_rs::prelude::*;
use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::{ImageEncoder, ImageReader};
use image::codecs::webp::WebPEncoder;
use image::imageops::FilterType;
use webp::{Encoder, WebPConfig};

pub enum ImageType {
    PNG,
    JPEG,
    GIF,
    BMP,
    WEBP,
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<ImageScript>()
}

#[php_class]
pub struct ImageScript {
    output_type: Option<ImageType>,
}

#[php_impl]
impl ImageScript {
    pub fn __construct() -> ImageScript {
        Self {
            output_type: None,
        }
    }

    pub fn transform_fast(&self, source_path: &str, dest_path: &str) {
        let reader = ImageReader::open(source_path)
            .unwrap()
            .with_guessed_format()
            .unwrap();

        let _format = reader.format().unwrap();
        let src_image = reader.decode().unwrap();

        // Create container for data of destination image
        let dst_width = 500;
        let dst_height = 500;
        let mut dst_image = Image::new(
            dst_width,
            dst_height,
            src_image.pixel_type().unwrap(),
        );

        // Create Resizer instance and resize source image
        // into buffer of destination image
        let mut resizer = Resizer::new();
        let options = ResizeOptions::new().resize_alg(ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3)).use_alpha(false);
        resizer.resize(&src_image, &mut dst_image, Some(&options)).unwrap();

        // Write destination image as PNG-file
        let mut writer = BufWriter::new(std::fs::File::create(dest_path).unwrap());
        WebPEncoder::new_lossless(&mut writer)
            .write_image(
                dst_image.buffer(),
                dst_width,
                dst_height,
                src_image.color().into(),
            )
            .unwrap();
    }

    pub fn transform(&self, source_path: &str, dest_path: &str) {
        let reader = ImageReader::open(source_path)
            .unwrap()
            .with_guessed_format()
            .unwrap();

        let src_image = reader.decode().unwrap();
        let dst_image = src_image.resize_exact(500, 500, FilterType::Lanczos3);

        // Write destination image as PNG-file
        let mut writer = BufWriter::new(std::fs::File::create(dest_path).unwrap());
        WebPEncoder::new_lossless(&mut writer)
            .write_image(
                dst_image.as_bytes(),
                dst_image.width(),
                dst_image.height(),
                dst_image.color().into(),
            )
            .unwrap();
    }

    pub fn transform_webp(&self, source_path: &str, dest_path: &str) {
        let reader = ImageReader::open(source_path)
            .unwrap()
            .with_guessed_format()
            .unwrap();

        let src_image = reader.decode().unwrap();

        // Create container for data of destination image
        let dst_width = 500;
        let dst_height = 500;
        let mut dst_image = Image::new(
            dst_width,
            dst_height,
            src_image.pixel_type().unwrap(),
        );

        // Create Resizer instance and resize source image
        // into buffer of destination image
        let mut resizer = Resizer::new();
        let options = ResizeOptions::new().resize_alg(ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3)).use_alpha(false);
        resizer.resize(&src_image, &mut dst_image, Some(&options)).unwrap();

        // Write destination image as PNG-file

        // Write destination image as PNG-file
        // let mut writer = BufWriter::new(std::fs::File::create(dest_path).unwrap());
        // PngEncoder::new(&mut writer)
        //     .write_image(
        //         dst_image.as_bytes(),
        //         dst_image.width(),
        //         dst_image.height(),
        //         dst_image.color().into(),
        //     )
        //     .unwrap();

        // Create the WebP encoder for the above image

        let encoder = Encoder::from_rgb(dst_image.buffer(), dst_width, dst_height);
        // Encode the image at a specified quality 0-100
        let mut config = WebPConfig::new().unwrap();
        config.near_lossless = 90;
        config.method = 6;
        config.low_memory = 1;
        let webp = encoder.encode_advanced(&config).unwrap();
        // Define and write the WebP-encoded file to a given path
        std::fs::write(dest_path, &*webp).unwrap();
    }
}