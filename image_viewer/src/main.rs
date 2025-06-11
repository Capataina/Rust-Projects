mod ascii_downscaler;
mod brightness_to_ascii_mapper;
mod calculate_brightness;
mod image_loader;
mod parse_bmp_metadata;
mod parse_bmp_pixel_data;
mod render_ascii_image;

use crate::ascii_downscaler::ascii_downscaler;
use crate::brightness_to_ascii_mapper::brightness_to_ascii_mapper;
use crate::calculate_brightness::calculate_brightness;
use crate::parse_bmp_metadata::BMPMetadata;
use crate::parse_bmp_pixel_data::{parse_bmp_pixel_data, BMPImage};
use crate::render_ascii_image::render_ascii_image;
use image_loader::{image_loader, ImageStruct};
use parse_bmp_metadata::parse_bmp_metadata;
use std::path::PathBuf;

fn main() {
    // let all_args: Vec<String> = env::args().collect();

    // if all_args.len() == 1 {
    //     println!("Usage: image-loader <image-file>");
    //     return;
    // }

    // let image_path: &String = &all_args[1];
    // let image_path_literal = PathBuf::from(&image_path);

    let image_path_literal = PathBuf::from(r"C:\Rust-Projects\image_viewer\example_image.bmp");

    // This is not needed yet, we will use this once we need more things for the command to work.
    // let user_args: &[String] = &all_args[1..];

    let image: ImageStruct = image_loader(image_path_literal.to_str().unwrap());

    // println!("{}", image.name);
    // println!("{}", image.extension);

    let debug_metadata: BMPMetadata = parse_bmp_metadata::parse_bmp_metadata(&image.image);
    // println!("{:?}", debug.bits_per_pixel);

    // let row_stride = ((debug.bits_per_pixel * u16::try_from(debug.width).unwrap() + 31) / 32) * 4;
    // println!("{:?}", row_stride);

    let debug_pixeldata: BMPImage = parse_bmp_pixel_data(&image.image, debug_metadata);

    println!("{:?}", &debug_pixeldata.pixels[..12]);

    let image_metadata = parse_bmp_metadata(&image.image);
    let image_pixel_data = parse_bmp_pixel_data(&image.image, image_metadata);
    let image_brightness_data = calculate_brightness(image_pixel_data);
    let image_downscaled_brightness_data = ascii_downscaler(image_brightness_data, 75, 30);
    let image_ascii_map = brightness_to_ascii_mapper(image_downscaled_brightness_data);
    let image_ascii = render_ascii_image(image_ascii_map);

    println!("{}", image_ascii)
}
