# Notes

## Software Rendering

Comment from Olivier Goffart about software rendering with Slint:

> The Software renderer can also be used: [1] (With a custom Platform [2])
>
> There is also Window::take_snapshot [3]

[1]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.SoftwareRenderer.html#method.render
[2]: https://docs.rs/slint/latest/slint/platform/index.html
[3]: https://docs.rs/slint/latest/slint/struct.Window.html#method.take_snapshot

**Software Rendering Flow**

1. [`fn set_platform(Box<dyn Platform>) -> Result<(), SetPlatformError>`][fn.set_platform]
1. [`trait Platform`][trait.Platform]
  - [`fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError>`][trait.Platform.create_window_adapter]
1. [`trait WindowAdapter`][trait.WindowAdapter]
  - [`fn renderer(&self) -> &dyn Renderer`][trait.WindowAdapter.renderer]
  - [`struct MinimalSoftwareWindow`][struct.MinimalSoftwareWindow]
    - [`impl WindowAdapter`][struct.MinimalSoftwareWindow.impl.WindowAdapter]
1. [`trait Renderer`][trait.Renderer]
  - `impl<T: RendererSealed> Renderer for T`
1. [`trait RendererSealed`][trait.RendererSealed]
1. [`struct SoftwareRenderer`][struct.SoftwareRenderer]
  - [`impl Renderer`][struct.SoftwareRenderer.impl.Renderer]
    - The `impl RendererSealed` is `#[doc(hidden)]` but we have the blanket impl for `Renderer`
  - [`fn render(&self, &mut [impl TargetPixel], usize)`][struct.SoftwareRenderer.render]

[fn.set_platform]: https://docs.rs/slint/latest/slint/platform/fn.set_platform.html
[trait.Platform]: https://docs.rs/slint/latest/slint/platform/trait.Platform.html
[trait.Platform.create_window_adapter]: https://docs.rs/slint/latest/slint/platform/trait.Platform.html#tymethod.create_window_adapter
[trait.WindowAdapter]: https://docs.rs/slint/latest/slint/platform/trait.WindowAdapter.html
[trait.WindowAdapter.renderer]: https://docs.rs/slint/latest/slint/platform/trait.WindowAdapter.html#tymethod.renderer
[struct.MinimalSoftwareWindow]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.MinimalSoftwareWindow.html
[struct.MinimalSoftwareWindow.impl.WindowAdapter]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.MinimalSoftwareWindow.html#impl-WindowAdapter-for-MinimalSoftwareWindow
[trait.Renderer]: https://docs.rs/slint/latest/slint/platform/trait.Renderer.html
[trait.RendererSealed]: https://docs.rs/i-slint-core/1.9.2/i_slint_core/renderer/trait.RendererSealed.html
[struct.SoftwareRenderer]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.SoftwareRenderer.html
[struct.SoftwareRenderer.impl.Renderer]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.SoftwareRenderer.html#impl-Renderer-for-T
[struct.SoftwareRenderer.render]: https://docs.rs/slint/latest/slint/platform/software_renderer/struct.SoftwareRenderer.html#method.render
