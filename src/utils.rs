use std::str::FromStr;

use imageproc::image;
use palette::Srgb;

use crate::error::{AppError, Result};

pub fn parse_hex_string(hex: &str) -> Result<image::Rgba<u8>> {
    let srgb = palette::Srgb::from_str(hex)
        .map_err(|_| AppError::Message(format!("invalid color: {hex}")))?;
    Ok(srgb.to_pixel())
}

pub trait PaletteToImagePixel<T> {
    fn to_pixel(self) -> image::Rgba<T>;
}

impl PaletteToImagePixel<u8> for Srgb<u8> {
    fn to_pixel(self) -> image::Rgba<u8> {
        image::Rgba([self.red, self.green, self.blue, u8::MAX])
    }
}

impl PaletteToImagePixel<f32> for Srgb<f32> {
    fn to_pixel(self) -> image::Rgba<f32> {
        image::Rgba([self.red, self.green, self.blue, 1.0])
    }
}
