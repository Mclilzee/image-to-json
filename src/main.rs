use image::{DynamicImage, GenericImageView, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("File path was not specified");
    let image = image::open(path).unwrap();
    let rbg = extract_json(image);
    println!("{}", rbg);
}

fn extract_json(image: DynamicImage) -> String {
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
        "{{\"r\":{}, \"g\":{}, \"b\":{}}}",
        pixel[0], pixel[1], pixel[2]
    );
    return string;
}
