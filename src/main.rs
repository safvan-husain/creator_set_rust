mod common;

extern crate ab_glyph;
extern crate image;
extern crate imageproc;
extern crate rusttype;
use common::draw_text_on_image;
use image::{open, GenericImage, Rgba};
use imageproc::drawing::Canvas;
use std::env::current_dir;
use std::path::Path;
fn main() {
    let mut current_di = current_dir().unwrap();
    let mut output_dir = current_dir().unwrap();
    current_di.push("assets");
    current_di.push("white.png");
    output_dir.push("assets");
    output_dir.push("output.png");

    // let image_path = Path::new(&current_di);
    // let mut img = open(image_path).unwrap().into_rgba8();
    let image_path = Path::new(&current_di);
    let mut img = open(image_path).unwrap().into_rgba8();
    // let mut new_image: RgbaImage = RgbaImage::new(img.width(), img.height());
    let translucent_red = Rgba([255u8, 0u8, 0u8, 127u8]);
    write_a_pixel(&mut img, translucent_red);
    draw_text_on_image(&mut img, "Hello world", 12f32, translucent_red);
    // Specify the path where you want to save the image
    let save_path = Path::new(&output_dir);
    img.save(save_path).expect("Failed to save image");
}

fn write_a_pixel<C: GenericImage>(canvas: &mut C, c: C::Pixel) {
    let h = canvas.height();

    let w = canvas.width();

    for i in 5..h {
        canvas.draw_pixel(0, i, c);
    }

    println!("writed pixel");
}
