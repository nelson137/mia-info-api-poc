// Prevent console window in addition to Slint window in Windows release builds
// when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use imageproc::image;
use slint::platform::WindowEvent;

static MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR");

slint::slint! {
    import { VerticalBox } from "std-widgets.slint";

    export component Badge inherits Window {
        in property <string> text;

        width: 305px;

        Rectangle {
            background: #ff4040;

            Text {
                color: black;
                font-size: 72px;
                text: root.text;
            }
        }
    }
}

static TEXT: &str = "11.22.33";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = Badge::new()?;

    let text = slint::SharedString::from(TEXT);
    ui.set_text(text);

    let thread = {
        let ui = ui.as_weak();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("📸 snapshot");
            ui.upgrade_in_event_loop(|ui| {
                snapshot(ui.window()).unwrap();
                ui.window().dispatch_event(WindowEvent::CloseRequested);
            })
        })
    };

    ui.run()?;

    thread.join().expect("join")?;

    Ok(())
}

fn snapshot(window: &slint::Window) -> Result<(), slint::PlatformError> {
    let pixel_buffer = window.take_snapshot()?;

    let mut image = image::RgbaImage::new(pixel_buffer.width(), pixel_buffer.height());

    for (in_pixel, out_pixel) in pixel_buffer.as_slice().iter().zip(image.pixels_mut()) {
        let slint::Rgba8Pixel { r, g, b, a } = *in_pixel;
        *out_pixel = image::Rgba::<u8>([r, g, b, a]);
    }

    let image_path = format!("{MANIFEST_DIR}/../../target/badge-slint.png");
    image
        .save(&image_path)
        .map_err(|err| slint::PlatformError::OtherError(Box::new(err)))?;

    println!("💾 image saved");

    Ok(())
}
