use image::{DynamicImage, GenericImageView, Rgb};

fn main() {
    let image = image::open("./images/cropped.jpg").unwrap();
}

fn extract_string(pixel: &Rgb<u8>) -> String {
    let pixel = pixel.0;
    let string = format!("{{r:{}, g:{}, b:{}}}", pixel[0], pixel[1], pixel[2]);
    return string;
}

fn extract_image(image: DynamicImage) -> String {
    let mut string = String::from("[");
    let (x, y) = image.dimensions();
    let rbg = image.to_rgb8();

    for i in 0..x {
        for j in 0..y {
            let pixel = rbg.get_pixel(i, j);
            string = format!("{}, {}", string, extract_string(pixel));
        }
    }
    string += "]";
    return string;
}
