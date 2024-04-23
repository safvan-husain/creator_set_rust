mod common;
extern crate ab_glyph;
extern crate ffmpeg_next;
extern crate image;
use std::io::Cursor;
extern crate imageproc;
extern crate rusttype;
extern crate video_rs;
use common::draw_text_on_image;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::util::frame::Video;
use image::io::Reader as ImageReader;
use image::{open, GenericImage, ImageBuffer, Rgba};
use imageproc::drawing::Canvas;
use std::env::current_dir;
use std::ops::Range;
use std::path::Path;
use video_rs::decode::Decoder;
fn main() {
    let mut _current_di = current_dir().unwrap();
    let mut decodere = match Decoder::new(Path::new("assets\\white.mp4")) {
        Ok(decoder) => {
            println!("success decoding");
            decoder
        }
        Err(e) => {
            eprintln!("Failed to create decoder: {}", e);
            return;
        }
    };
    let number_of_frames = decodere.frames().unwrap();
    println!("number of frames {number_of_frames}");
    let raw_frames_iter = decodere.decode_raw_iter();
    for result in raw_frames_iter {
        match result {
            Ok(raw_frame) => {
                println!("processed a fame");

                // Process the valid RawFrame
            }
            Err(error) => {
                println!("Ocurred a error {error}");
                return;
                // Handle the error
            }
        }
    }

    // let mut output_dir = current_dir().unwrap();
    // current_di.push("assets");
    // current_di.push("white.png");
    // output_dir.push("assets");
    // output_dir.push("output.png");

    // let mut img = open(image_path).unwrap().into_rgba8();
    // let translucent_red = Rgba([255u8, 0u8, 0u8, 127u8]);
    // write_a_pixel(&mut img, translucent_red);
    // draw_text_on_image(&mut img, "Hello world", 12f32, translucent_red);
    // let save_path = Path::new(&output_dir);
    // img.save(save_path).expect("Failed to save image");
}

fn write_a_pixel<C: GenericImage>(canvas: &mut C, c: C::Pixel) {
    let h = canvas.height();

    let w = canvas.width();

    for i in 5..h {
        canvas.draw_pixel(0, i, c);
    }

    println!("writed pixel");
}
fn save_video_frame_as_image(
    video: &mut Video,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Assuming the video frame is in RGBA format
    let format = video.format();
    if format != Pixel::RGBA {
        return Err("Unsupported pixel format for saving as image.".into());
    }

    // Get the frame's dimensions
    let width = video.width() as usize;
    let height: usize = video.height() as usize;

    // Convert the frame's data to an ImageBuffer
    let mut image_data: Vec<Rgba<u8>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let m: Range<usize> = (y * width + x) * 4..(y * width + x) * 4 + 4;
            let pixel = video.data_mut(0).get(m).unwrap();
            image_data.push(Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
        }
    }
    let image = create_image_from_vec(width as u32, height as u32, image_data);
    // Create an ImageBuffer from the pixel data

    Ok(())
}

fn create_image_from_vec(width: u32, height: u32, data: Vec<Rgba<u8>>) -> image::DynamicImage {
    // Ensure the data vector has the correct size
    assert_eq!(data.len(), (width * height) as usize);
    // Convert Vec<Rgba<u8>> to Vec<u8>

    // Create an ImageBuffer from the data
    let image_buffer = ImageBuffer::from_raw(width, height, data).unwrap();

    // Convert the ImageBuffer to a DynamicImage
    image::DynamicImage::ImageRgba8(image_buffer)
}
