use crate::parse_bmp_pixel_data::BMPImage;

pub struct ImageBrightnessData {
    pub image_width: u32,
    pub image_height: u32,
    pub brightness_array: Vec<f64>,
}
pub fn calculate_brightness(bmp_pixel_data: BMPImage) -> ImageBrightnessData {
    let all_pixels = bmp_pixel_data.pixels;
    let mut pixel_brightness_array: Vec<f64> =
        Vec::with_capacity((bmp_pixel_data.height * bmp_pixel_data.width / 4) as usize);

    for pixel in (0..all_pixels.len()).step_by(4) {
        let red_pixel = all_pixels[pixel] as f64;
        let green_pixel = all_pixels[pixel + 1] as f64;
        let blue_pixel = all_pixels[pixel + 2] as f64;

        let mut pixel_brightness = red_pixel * 0.2126 + green_pixel * 0.7152 + blue_pixel * 0.0722;

        pixel_brightness *= (all_pixels[pixel + 3] as f64) / 255.0;

        pixel_brightness_array.push(pixel_brightness);
    }

    ImageBrightnessData {
        image_width: bmp_pixel_data.width,
        image_height: bmp_pixel_data.height,
        brightness_array: pixel_brightness_array,
    }
}
