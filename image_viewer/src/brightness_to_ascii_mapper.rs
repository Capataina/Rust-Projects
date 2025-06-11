use crate::calculate_brightness::ImageBrightnessData;

pub struct ImageASCIIMapping {
    pub image_width: u32,
    pub image_height: u32,
    pub ascii_mapping: Vec<char>,
}

pub fn brightness_to_ascii_mapper(image_brightness_data: ImageBrightnessData) -> ImageASCIIMapping {
    let ascii_characters: Vec<char> = vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    let mut ascii_pixels: Vec<char> =
        Vec::with_capacity(image_brightness_data.brightness_array.len());

    for pixel_brightness in image_brightness_data.brightness_array.iter() {
        let normalised_pixel_brightness =
            (pixel_brightness / 255.0) * (ascii_characters.len() as f64 - 1.0);

        ascii_pixels.push(ascii_characters[normalised_pixel_brightness.floor() as usize]);
    }

    ImageASCIIMapping {
        image_width: image_brightness_data.image_width,
        image_height: image_brightness_data.image_height,
        ascii_mapping: ascii_pixels,
    }
}
