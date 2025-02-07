// Prevent console window in addition to Slint window in Windows release builds
// when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;

use i_slint_core::{
    item_rendering::RepaintBufferType,
    software_renderer::{MinimalSoftwareWindow, SoftwareRenderer},
};
use imageproc::image;
use slint::{
    LogicalSize, PlatformError, Rgba8Pixel, SharedPixelBuffer,
    platform::{EventLoopProxy, Platform, WindowAdapter, WindowEvent, set_platform},
};

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

#[derive(Clone)]
struct CustomPlatform {
    inner: Rc<i_slint_backend_winit::Backend>,
    window: Rc<MinimalSoftwareWindow>,
}

impl CustomPlatform {
    fn new(window: Rc<MinimalSoftwareWindow>) -> Result<Self, PlatformError> {
        Ok(Self {
            inner: Rc::new(i_slint_backend_winit::Backend::new()?),
            window,
        })
    }
}

impl Platform for CustomPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        // self.inner.create_window_adapter()
        Ok(self.window.clone())
    }

    fn run_event_loop(&self) -> Result<(), PlatformError> {
        self.inner.run_event_loop()
    }

    fn new_event_loop_proxy(&self) -> Option<Box<dyn EventLoopProxy>> {
        self.inner.new_event_loop_proxy()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    window.set_size(LogicalSize::new(500., 500.));

    let custom_platform = CustomPlatform::new(window.clone())?;
    set_platform(Box::new(custom_platform.clone())).expect("backend already initialized");

    let ui = Badge::new()?;

    let text = slint::SharedString::from(TEXT);
    ui.set_text(text);

    ui.show()?;

    window.draw_if_needed(|renderer| {
        use i_slint_core::renderer::RendererSealed;
        let buffer = renderer.take_snapshot().unwrap();
        save_snapshot(buffer).unwrap();
    });

    // let window_adapter = custom_platform.create_window_adapter()?;
    // let renderer = window_adapter.renderer();
    // renderer.set_window_adapter(&window_adapter);
    // save_snapshot(renderer.take_snapshot()?)?;

    // let thread = {
    //     let ui = ui.as_weak();
    //     std::thread::spawn(move || {
    //         std::thread::sleep(std::time::Duration::from_millis(500));
    //         println!("📸 snapshot");
    //         ui.upgrade_in_event_loop(|ui| {
    //             let buffer = ui.window().take_snapshot().unwrap();
    //             save_snapshot(buffer).unwrap();
    //             ui.window().dispatch_event(WindowEvent::CloseRequested);
    //         })
    //     })
    // };
    // ui.run()?;
    // thread.join().expect("join")?;

    Ok(())
}

fn save_snapshot(pixel_buffer: SharedPixelBuffer<Rgba8Pixel>) -> Result<(), PlatformError> {
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
