use ab_glyph::FontRef;
use anyhow::Result;
use imageproc::{
    drawing,
    image::{Rgb, RgbImage},
};

static TEXT: &str = "█ O_O █";

static FONT_BYTES: &[u8] = include_bytes!("DejaVuSans.ttf");
const FONT_SCALE: f32 = 128.0;

const TEXT_MARGIN: u32 = 8;

const RED: Rgb<u8> = Rgb([255_u8, 64, 64]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

static MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR");

fn main() {
    if let Err(err) = main_() {
        eprintln!("{err}");
    }
}

fn main_() -> Result<()> {
    let font = FontRef::try_from_slice(FONT_BYTES)?;

    let (text_width, text_height) = drawing::text_size(FONT_SCALE, &font, TEXT);

    let mut image = RgbImage::from_pixel(
        text_width + 2 * TEXT_MARGIN,
        text_height + 2 * TEXT_MARGIN,
        RED,
    );

    let text_x = (image.width() / 2) as i32 - (text_width / 2) as i32;
    let text_y = (image.height() / 2) as i32 - (text_height / 2) as i32;

    drawing::draw_text_mut(&mut image, BLACK, text_x, text_y, FONT_SCALE, &font, TEXT);

    let image_path = format!("{MANIFEST_DIR}/target/badge-imageproc.png");
    image.save(&image_path)?;

    Ok(())
}
