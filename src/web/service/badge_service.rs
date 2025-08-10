use std::{io::Cursor, sync::Arc};

use ab_glyph::FontRef;
use anyhow::Result;
use imageproc::{drawing, image};
use mia_info_poc_macros::substate;

use crate::web::state::AppState;

use super::Service;

#[substate(AppState, field(badge_service))]
#[cfg_attr(test, mockall::automock)]
pub trait BadgeService: Service {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn generate_count_badge(&self, count: u32) -> Result<Vec<u8>>;
    fn generate_version_badge(&self, version: &str) -> Result<Vec<u8>>;
}

#[cfg(test)]
impl From<MockBadgeService> for axum::extract::State<std::sync::Arc<dyn BadgeService>> {
    fn from(value: MockBadgeService) -> Self {
        axum::extract::State(std::sync::Arc::new(value))
    }
}

static FONT_BYTES: &[u8] = include_bytes!("../../../examples/DejaVuSans.ttf");

const FONT_SCALE: f32 = 128.0;
const TEXT_MARGIN: u32 = 8;

#[derive(Clone)]
pub struct ImageProcBadgeService {
    font: Arc<FontRef<'static>>,
}

impl BadgeService for ImageProcBadgeService {
    fn new() -> Result<Self> {
        let font = Arc::new(FontRef::try_from_slice(FONT_BYTES)?);
        Ok(Self { font })
    }

    fn generate_count_badge(&self, count: u32) -> Result<Vec<u8>> {
        const MAGENTA: image::Rgba<u8> = image::Rgba([255_u8, 64, 255, 255]);
        const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
        let count = count.to_string();
        generate_badge(MAGENTA, BLACK, &self.font, FONT_SCALE, TEXT_MARGIN, &count)
    }

    fn generate_version_badge(&self, version: &str) -> Result<Vec<u8>> {
        const RED: image::Rgba<u8> = image::Rgba([255_u8, 64, 64, 255]);
        const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
        generate_badge(RED, BLACK, &self.font, FONT_SCALE, TEXT_MARGIN, version)
    }
}

fn generate_badge<P>(
    bg: P,
    fg: P,
    font: &FontRef,
    font_scale: f32,
    margin: u32,
    text: &str,
) -> Result<Vec<u8>>
where
    P: image::Pixel<Subpixel = u8> + image::PixelWithColorType,
{
    let (text_width, text_height) = drawing::text_size(font_scale, font, text);

    let mut image =
        image::ImageBuffer::from_pixel(text_width + 2 * margin, text_height + 2 * margin, bg);

    let text_x = (image.width() / 2) as i32 - (text_width / 2) as i32;
    let text_y = (image.height() / 2) as i32 - (text_height / 2) as i32;

    drawing::draw_text_mut(&mut image, fg, text_x, text_y, font_scale, font, text);

    format_image_buffer(image)
}

fn format_image_buffer<P>(image: image::ImageBuffer<P, Vec<u8>>) -> Result<Vec<u8>>
where
    P: image::Pixel<Subpixel = u8> + image::PixelWithColorType,
{
    let mut formatted_image = Vec::<u8>::new();
    let mut writer = Cursor::new(&mut formatted_image);
    image.write_to(&mut writer, image::ImageFormat::Png)?;
    Ok(formatted_image)
}
