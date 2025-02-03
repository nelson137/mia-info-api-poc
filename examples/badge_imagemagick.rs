use anyhow::Result;
use magick_rust::{DrawingWand, GravityType, MagickWand, PixelWand};

static TEXT: &str = "█ O_O █";

const BADGE_WIDTH: usize = 512;
const BADGE_HEIGHT: usize = 256;

const FONT_SIZE: f64 = 100.0;

static MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR");

fn main() {
    magick_rust::magick_wand_genesis();

    if let Err(err) = main_() {
        eprintln!("{err}");
    }

    magick_rust::magick_wand_terminus();
}

fn main_() -> Result<()> {
    let mut pixel_wand = PixelWand::new();
    pixel_wand.set_red(1.0);
    pixel_wand.set_green(0.25);
    pixel_wand.set_blue(0.25);

    let mut magick_wand = MagickWand::new();
    magick_wand.new_image(BADGE_WIDTH, BADGE_HEIGHT, &pixel_wand)?;

    let mut drawing_wand = DrawingWand::new();
    drawing_wand.set_font_size(FONT_SIZE);
    drawing_wand.set_gravity(GravityType::Center);

    magick_wand.annotate_image(&drawing_wand, 0., 0., 0., TEXT)?;

    let image_path = format!("{MANIFEST_DIR}/target/badge-imagemagick.png");
    magick_wand.write_image(image_path.as_str())?;

    Ok(())
}
