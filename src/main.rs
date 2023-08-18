use image::{DynamicImage, GenericImageView, Rgb};
use std::fs::read_dir;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let images = match get_images(args) {
        Ok(images) => images,
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    };

    let json = images
        .iter()
        .map(|image| String::from(extract_json(image)))
        .collect::<Vec<String>>()
        .join(",\n");

    if images.len() <= 1 {
        println!("{}", json);
    } else {
        println!("[{}]", json);
    }
}

fn get_images(args: Vec<String>) -> Result<Vec<DynamicImage>, String> {
    let mut images: Vec<DynamicImage> = Vec::new();
    let file = match args.get(1) {
        Some(file) => file,
        None => return Err(String::from("Error: File path not found")),
    };

    if Path::new(file).is_dir() {
        images = match read_dir(file) {
            Ok(dir) => dir,
            Err(_) => return Err(String::from("Error: Could not open file or directory")),
        }
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(|file_name| image::open(file_name).ok())
        .collect();
    } else {
        let image = match image::open(file) {
            Ok(image) => image,
            Err(_) => return Err(String::from("Failed parsing image")),
        };

        images.push(image);
    }

    return Ok(images);
}

fn extract_json(image: &DynamicImage) -> String {
    let mut image_strings: Vec<String> = Vec::new();
    let (x, y) = image.dimensions();
    let rbg = image.to_rgb8();

    for i in 0..x {
        for j in 0..y {
            let pixel = rbg.get_pixel(i, j);
            image_strings.push(extract_string(pixel));
        }
    }

    let objects_string = image_strings.join(", ");

    return format!("[{}]", objects_string);
}

fn extract_string(pixel: &Rgb<u8>) -> String {
    let pixel = pixel.0;
    let string = format!(
        "{{ \"r\" : {}, \"g\" : {}, \"b\" : {}}}",
        pixel[0], pixel[1], pixel[2]
    );
    return string;
}
