use std::path::PathBuf;

pub struct ImageStruct {
    pub path: PathBuf,
    pub name: String,
    pub extension: String,
    pub image: Vec<u8>,
}

pub fn image_loader(absolute_image_path: &str) -> ImageStruct {
    let image_path_string: String = String::from(absolute_image_path);
    let image_path = std::path::Path::new(&image_path_string).to_path_buf();
    let image_bits: Vec<u8> = std::fs::read(&image_path).unwrap();

    let image_details: Vec<&str> = image_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .rsplitn(2, ".")
        .collect::<Vec<&str>>();

    let image_name = image_details[1].to_string();
    let image_extension = image_details[0].to_string();

    ImageStruct {
        path: image_path,
        name: image_name,
        extension: image_extension,
        image: image_bits,
    }
}
