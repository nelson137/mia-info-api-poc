// Prevent console window in addition to Slint window in Windows release builds
// when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;

use anyhow::Result;
use i_slint_core::{
    graphics::euclid, item_rendering::RepaintBufferType, lengths::LogicalRect,
    renderer::RendererSealed, software_renderer::MinimalSoftwareWindow,
};
use imageproc::image;
use slint::platform;

const IMAGE_OUT_DIR: &str = concat!(std::env!("CARGO_MANIFEST_DIR"), "/../../target");

slint::slint! {
    import { VerticalBox } from "std-widgets.slint";

    component Badge {
        in property <length> badge-padding-block;
        in property <length> badge-padding-inline;
        in property <brush> badge-background <=> rect.background;
        in property <brush> badge-color <=> label.color;
        in property <length> badge-font-size <=> label.font-size;
        in property <string> badge-text <=> label.text;

        VerticalLayout {
            alignment: start;

            HorizontalLayout {
                alignment: start;

                rect := Rectangle {
                    background: blue; // default

                    rect-inner := HorizontalLayout {
                        padding-top <=> root.badge-padding-block;
                        padding-bottom <=> root.badge-padding-block;
                        padding-left <=> root.badge-padding-inline;
                        padding-right <=> root.badge-padding-inline;

                        label := Text {
                            color: white; // default
                            font-size: 48px; // default
                        }
                    }
                }
            }
        }
    }

    export component BadgeUi inherits Window {
        in property <length> badge-padding-block <=> badge.badge-padding-block;
        in property <length> badge-padding-inline <=> badge.badge-padding-inline;
        in property <length> badge-font-size <=> badge.badge-font-size;
        in property <brush> badge-color <=> badge.badge-color;
        in property <brush> badge-background <=> badge.badge-background;
        in property <string> badge-text <=> badge.badge-text;

        out property <int> badge_width : badge.width / 1px;
        out property <int> badge_height : badge.height / 1px;

        badge := Badge {}
    }
}

static TEXT: &str = "0xdeadbeef";

#[derive(Clone)]
struct SoftwareRendererPlatform {
    window: Rc<MinimalSoftwareWindow>,
}

type PlatformResult<T> = std::result::Result<T, slint::PlatformError>;

impl SoftwareRendererPlatform {
    fn new(window: Rc<MinimalSoftwareWindow>) -> PlatformResult<Box<Self>> {
        Ok(Box::new(Self { window }))
    }
}

impl platform::Platform for SoftwareRendererPlatform {
    fn create_window_adapter(&self) -> PlatformResult<Rc<dyn platform::WindowAdapter>> {
        Ok(self.window.clone())
    }
}

fn init() -> Result<Rc<MinimalSoftwareWindow>> {
    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    // Initial size doesn't matter, we'll resize after rendering once we know
    // the exact size
    // window.set_size(slint::PhysicalSize::new(0, 0));

    platform::set_platform(SoftwareRendererPlatform::new(window.clone())?)?;

    Ok(window)
}

fn main() -> Result<()> {
    let window = init()?;

    // Create badge

    let ui = BadgeUi::new()?;

    ui.set_badge_padding_block(10.);
    ui.set_badge_padding_inline(16.);
    ui.set_badge_background(slint::Color::from_argb_u8(255, 255, 90, 0).into());
    ui.set_badge_color(slint::Color::from_argb_u8(255, 10, 10, 180).into());
    ui.set_badge_font_size(48.);

    let text_arg = std::env::args().nth(1);
    let text = slint::SharedString::from(text_arg.as_deref().unwrap_or(TEXT));
    ui.set_badge_text(text);

    // Shrink window to fit badge

    let badge_size = match (ui.get_badge_width(), ui.get_badge_height()) {
        (w, h) if w > 0 && h > 0 => slint::PhysicalSize::new(w as _, h as _),
        _ => anyhow::bail!("UI width and height are negative"),
    };
    window.set_size(badge_size);

    // Render to buffer

    let buffer = screenshot(window.clone());

    // Write buffer to PNG file

    save_screenshot(buffer)?;

    Ok(())
}

pub fn screenshot(window: Rc<MinimalSoftwareWindow>) -> slint::SharedPixelBuffer<slint::Rgb8Pixel> {
    let size = window.size();
    let (width, height) = (size.width, size.height);

    let mut buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(width, height);

    window.request_redraw();
    window.draw_if_needed(|renderer| {
        let dirty_rect = LogicalRect::from_size(euclid::size2(width as f32, height as f32));
        renderer.mark_dirty_region(dirty_rect.into());
        renderer.render(buffer.make_mut_slice(), width as usize);
    });

    eprintln!("📸 snapshot taken ({width} x {height})");

    // let mut buffer_with_alpha =
    //     slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(buffer.width(), buffer.height());
    //
    // for (target_pixel, source_pixel) in buffer_with_alpha
    //     .make_mut_slice()
    //     .iter_mut()
    //     .zip(buffer.as_slice().iter())
    // {
    //     *target_pixel.rgb_mut() = *source_pixel;
    // }

    buffer
}

fn save_screenshot(buffer: slint::SharedPixelBuffer<slint::Rgb8Pixel>) -> Result<()> {
    let mut image_path = std::path::Path::new(IMAGE_OUT_DIR).canonicalize()?;
    image_path.push("badge-slint.png");

    image::save_buffer(
        &image_path,
        buffer.as_bytes(),
        buffer.width(),
        buffer.height(),
        image::ColorType::Rgb8,
    )?;

    eprintln!("💾 snapshot saved: {}", image_path.display());

    Ok(())
}
