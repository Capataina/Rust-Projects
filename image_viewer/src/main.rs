mod image_loader;

use image_loader::{image_loader, ImageStruct};
use std::env;
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

    println!("{}", image.name);
    println!("{}", image.extension);
    println!("{:?}", image.image);
}
