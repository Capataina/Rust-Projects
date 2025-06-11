use crate::calculate_brightness::ImageBrightnessData;

pub fn ascii_downscaler(
    data: ImageBrightnessData,
    target_width: u32,
    target_height: u32,
) -> ImageBrightnessData {
    let mut downscaled_brightness: Vec<f64> =
        Vec::with_capacity((target_width * target_height) as usize);

    let original_width = data.image_width;
    let original_height = data.image_height;
    let original_pixels = data.brightness_array;

    let x_step = original_width as f64 / target_width as f64;
    let y_step = original_height as f64 / target_height as f64;

    for y in 0..target_height {
        for x in 0..target_width {
            let mut brightness_sum = 0.0;
            let mut pixel_count = 0;

            let x_start = (x as f64 * x_step) as usize;
            let y_start = (y as f64 * y_step) as usize;
            let x_end = ((x + 1) as f64 * x_step).min(original_width as f64) as usize;
            let y_end = ((y + 1) as f64 * y_step).min(original_height as f64) as usize;

            for yy in y_start..y_end {
                for xx in x_start..x_end {
                    let index = yy * original_width as usize + xx;
                    brightness_sum += original_pixels[index];
                    pixel_count += 1;
                }
            }

            let average_brightness = if pixel_count > 0 {
                brightness_sum / pixel_count as f64
            } else {
                0.0
            };

            downscaled_brightness.push(average_brightness);
        }
    }

    ImageBrightnessData {
        image_width: target_width,
        image_height: target_height,
        brightness_array: downscaled_brightness,
    }
}
