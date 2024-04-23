mod common;
extern crate ab_glyph;
extern crate ffmpeg_next;
extern crate image;
use std::io::{Cursor, Error, Write};
extern crate imageproc;
extern crate rusttype;
extern crate video_rs;
use common::draw_text_on_image;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::util::frame::Video;
use image::io::Reader as ImageReader;
use image::{open, DynamicImage, GenericImage, ImageBuffer, ImageError, Rgb};
use imageproc::drawing::Canvas;
use std::env::current_dir;
use std::fs::File;
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
    let mut raw_frames_iter = decodere.decode_raw_iter();
    let mut index = 0;
    // let index = raw_frames_iter.nth(0).unwrap().unwrap();
    // let result = save_file(&index, 0);
    for result in raw_frames_iter {
        match result {
            Ok(raw_frame) => {
                let result = save_file(&raw_frame, index);
                // let result = save_video_frame_as_image(&mut raw_frame);
                match result {
                    Ok(r) => {
                        println!("success");
                        index += 1;
                    }
                    Err(e) => println!("error saving {:?}", e),
                }
                // println!("processed a fame");

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
    // path: &str,
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
    let mut image_data: Vec<u8> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let m: Range<usize> = (y * width + x) * 4..(y * width + x) * 4 + 4;
            let pixel = video.data_mut(0).get(m).unwrap();
            image_data.push(pixel[0]);
        }
    }
    let image = create_image_from_vec(width as u32, height as u32, image_data);
    // Create an ImageBuffer from the pixel data
    let result = image.save(Path::new("assets/img.png"));
    match result {
        Ok(s) => println!("success saving image "),
        Err(e) => println!("{e}"),
    }

    Ok(())
}

fn create_image_from_vec(width: u32, height: u32, data: Vec<u8>) -> DynamicImage {
    //image::DynamicImage {
    // Ensure the data vector has the correct size
    assert_eq!(data.len(), (width * height) as usize);
    // Convert Vec<Rgba<u8>> to Vec<u8>

    // Convert Vec<Rgba<u8>> to &[Rgba<u8>]
    let data_slice = &data[..];

    // Create an ImageBuffer from the data
    let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, data).unwrap();

    // Convert the ImageBuffer to a DynamicImage
    image::DynamicImage::ImageRgb8(image_buffer)
}
fn save_file2(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(format!("frame{}.ppm", index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}
fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    // Assuming frame.data(0) returns a slice of u8 representing the image data
    // and frame.width() and frame.height() return the dimensions of the image.
    let width = frame.width();
    let height = frame.height();
    let data = frame.data(0);

    // Create an ImageBuffer from the raw data
    let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, data.to_vec()).unwrap();
    println!("{index}");
    let directory_path = "output/frames"; // Specify your directory path here
    let filename = format!("{}/frame{}.png", directory_path, index);
    let path = Path::new(&filename);

    // Ensure the directory exists before saving the file
    if let Err(e) = std::fs::create_dir_all(directory_path) {
        eprintln!("Failed to create directory: {}", e);
        return Err(Error::new(std::io::ErrorKind::AddrNotAvailable, ""));
    }
    // Save the image as a PNG file
    img.save(path);

    Ok(())
}
