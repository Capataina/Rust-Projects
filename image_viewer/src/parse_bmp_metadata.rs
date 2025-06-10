pub struct BMPMetadata {
    pub file_size: u32,
    pub pixel_data_offset: u32,
    pub dib_header_size: u32,

    pub width: i32,
    pub height: i32,

    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,

    pub image_size: u32,
    pub horizontal_resolution: i32,
    pub vertical_resolution: i32,

    pub colors_used: u32,
    pub important_colors: u32,
}

pub fn parse_bmp_metadata(bmp_bytes: Vec<u8>) -> BMPMetadata {
    let signature: [u8; 2] = bmp_bytes[0..2].try_into().unwrap();
    assert_eq!(signature, [66, 77], "Invalid BMP file signature");

    let file_size = u32::from_le_bytes(bmp_bytes[2..6].try_into().unwrap());
    let pixel_data_offset = u32::from_le_bytes(bmp_bytes[10..14].try_into().unwrap());

    let dib_header_size = u32::from_le_bytes(bmp_bytes[14..18].try_into().unwrap());
    let width = i32::from_le_bytes(bmp_bytes[18..22].try_into().unwrap());
    let height = i32::from_le_bytes(bmp_bytes[22..26].try_into().unwrap());
    let planes = u16::from_le_bytes(bmp_bytes[26..28].try_into().unwrap());
    let bits_per_pixel = u16::from_le_bytes(bmp_bytes[28..30].try_into().unwrap());
    let compression = u32::from_le_bytes(bmp_bytes[30..34].try_into().unwrap());
    let image_size = u32::from_le_bytes(bmp_bytes[34..38].try_into().unwrap());
    let horizontal_resolution = i32::from_le_bytes(bmp_bytes[38..42].try_into().unwrap());
    let vertical_resolution = i32::from_le_bytes(bmp_bytes[42..46].try_into().unwrap());
    let colors_used = u32::from_le_bytes(bmp_bytes[46..50].try_into().unwrap());
    let important_colors = u32::from_le_bytes(bmp_bytes[50..54].try_into().unwrap());

    BMPMetadata {
        file_size,
        pixel_data_offset,
        dib_header_size,
        width,
        height,
        planes,
        bits_per_pixel,
        compression,
        image_size,
        horizontal_resolution,
        vertical_resolution,
        colors_used,
        important_colors,
    }
}
