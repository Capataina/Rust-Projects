use crate::parse_bmp_metadata::BMPMetadata;

pub struct BMPImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub bytes_per_pixel: usize,
}

pub fn parse_bmp_pixel_data(bmp_bytes: Vec<u8>, bmp_metadata: BMPMetadata) -> BMPImage {
    let width: usize = bmp_metadata.width as usize;
    let height: usize = bmp_metadata.height as usize;
    let bits_per_pixel: usize = bmp_metadata.bits_per_pixel as usize;
    let bytes_per_pixel: usize = bits_per_pixel / 8;
    let pixel_data_offset: usize = bmp_metadata.pixel_data_offset as usize;

    let row_stride: usize = ((bits_per_pixel * width + 31) / 32) * 4;

    let mut pixels = Vec::with_capacity(width * height * bytes_per_pixel);

    for row in (0..height).rev() {
        let row_start = pixel_data_offset + row * row_stride;

        for col in 0..width {
            let pixel_offset = row_start + col * bytes_per_pixel;
            let pixel = &bmp_bytes[pixel_offset..pixel_offset + bytes_per_pixel];

            let b = pixel[0];
            let g = pixel[1];
            let r = pixel[2];

            pixels.push(r);
            pixels.push(g);
            pixels.push(b);

            if bytes_per_pixel == 4 {
                let a = pixel[3];
                pixels.push(a);
            }
        }
    }

    BMPImage {
        width: bmp_metadata.width as u32,
        height: bmp_metadata.height as u32,
        pixels,
        bytes_per_pixel,
    }
}
