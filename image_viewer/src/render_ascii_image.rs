use crate::brightness_to_ascii_mapper::ImageASCIIMapping;

pub fn render_ascii_image(ascii_image_data: ImageASCIIMapping) -> String {
    let mut all_rows: Vec<String> = Vec::new();

    for row_chunk in ascii_image_data
        .ascii_mapping
        .chunks(ascii_image_data.image_width as usize)
    {
        let row_string: String = row_chunk.iter().collect();
        all_rows.push(row_string);
    }

    all_rows.join("\n")
}
