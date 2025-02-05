use std::sync::Arc;

use ab_glyph::FontRef;
use anyhow::Result;
use imageproc::{
    drawing,
    image::{Rgba, RgbaImage},
};

#[cfg_attr(test, mockall::automock)]
pub trait BadgeService {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn generate_badge(&self, version: &str) -> Vec<u8>;
}

static FONT_BYTES: &[u8] = include_bytes!("../../../examples/DejaVuSans.ttf");

#[derive(Clone)]
pub struct ImageProcBadgeService {
    font: Arc<FontRef<'static>>,
}

impl BadgeService for ImageProcBadgeService {
    fn new() -> Result<Self> {
        let font = Arc::new(FontRef::try_from_slice(FONT_BYTES)?);
        Ok(Self { font })
    }

    fn generate_badge(&self, version: &str) -> Vec<u8> {
        const FONT_SCALE: f32 = 128.0;

        const TEXT_MARGIN: u32 = 8;

        const RED: Rgba<u8> = Rgba([255_u8, 64, 64, 255]);
        const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

        let (text_width, text_height) = drawing::text_size(FONT_SCALE, &*self.font, version);

        let mut image = RgbaImage::from_pixel(
            text_width + 2 * TEXT_MARGIN,
            text_height + 2 * TEXT_MARGIN,
            RED,
        );

        let text_x = (image.width() / 2) as i32 - (text_width / 2) as i32;
        let text_y = (image.height() / 2) as i32 - (text_height / 2) as i32;

        drawing::draw_text_mut(
            &mut image,
            BLACK,
            text_x,
            text_y,
            FONT_SCALE,
            &*self.font,
            version,
        );

        image.into_vec()
    }
}
