use image::{DynamicImage, GenericImageView, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("File path was not specified");
    let image = image::open(path).unwrap();
    let rbg = extract_image(image);
    println!("{}", rbg);
}

fn extract_image(image: DynamicImage) -> String {
    let mut string = String::from("[");
    let (x, y) = image.dimensions();
    let rbg = image.to_rgb8();

    for i in 0..x {
        for j in 0..y {
            let pixel = rbg.get_pixel(i, j);
            string = format!("{} {}", string, extract_string(pixel));
        }
    }
    string += "]";
    return string;
}

fn extract_string(pixel: &Rgb<u8>) -> String {
    let pixel = pixel.0;
    let string = format!("{{r:{}, g:{}, b:{}}},\n", pixel[0], pixel[1], pixel[2]);
    return string;
}
