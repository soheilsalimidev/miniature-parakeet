use std::{env, path::Path};

use colored::Colorize;
use image::{io::Reader as ImageReader, GenericImageView};

fn main() {
    let mut width = 200;
    termsize::get().map(|size| {
        width = size.cols;
    });
    let args: Vec<String> = env::args().collect();
    let img = ImageReader::open(Path::new(&args[1]))
        .unwrap()
        .decode()
        .unwrap()
        .resize(width.into(), 130, image::imageops::FilterType::Triangle);

    img.save("./1.png").unwrap();
    // println!("{}", &img.width());
    for (x, _y, pixel) in img.pixels() {
        let gray_scale = 0.21 * (*pixel.0.get(0).unwrap() as f32)
            + 0.72 * (*pixel.0.get(1).unwrap() as f32)
            + 0.07 * (*pixel.0.get(2).unwrap() as f32);
        print!(
            "{}",
            &get_character(gray_scale).to_string().truecolor(
                *pixel.0.get(0).unwrap(),
                *pixel.0.get(1).unwrap(),
                *pixel.0.get(2).unwrap()
            )
        );
        if x == img.width() - 1 {
            print!("\r\n");
        }
    }
}

fn get_character(scale: f32) -> char {
    let scale_char = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let index = ((scale as f32) * (((scale_char.chars().count() - 1) as f32) / 255.0)) as usize;
    scale_char.as_bytes()[index] as char
}
