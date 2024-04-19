extern crate ab_glyph;
use ab_glyph::{point, FontArc, GlyphId, PxScale};
use image::{open, GenericImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

pub fn draw_text_on_image(canvas: &mut RgbaImage, text: &str, font_size: f32, color: Rgba<u8>) {
    // Load the font
    let font_data = include_bytes!("..\\..\\assets\\fonts\\open-sans\\OpenSans-bold.ttf");
    let font = FontArc::try_from_vec(font_data.to_vec()).unwrap();

    // Create a scale for the font
    let scale = PxScale::from(font_size);

    // Specify the position where the text should start
    let start_point = point(10.0, 10.0);
    draw_text_mut(
        canvas,
        color,
        start_point.x as i32,
        start_point.y as i32,
        scale,
        &font,
        text,
    );
}
