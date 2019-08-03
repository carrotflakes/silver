extern crate image;

use std::env;

fn main() {
    let mut args = env::args();

    let img_path = "./image.png";

    let img: image::RgbImage = image::ImageBuffer::new(512, 512);

    let mut img = image::ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Rgb([0u8, 0u8, 0u8])
        } else {
            image::Rgb([255u8, 255u8, 255u8])
        }
    });

    img.save(img_path).unwrap();

    println!("done!");
}
