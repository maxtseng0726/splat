mod renderer;

use std::sync::Arc;

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use renderer::Renderer;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    title: String,
    width: u32,
    height: u32,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    font_system: FontSystem,
    swash_cache: SwashCache,
    buffer: Option<Buffer>,
    pixel_buffer: Vec<u8>,
}

impl App {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            window: None,
            renderer: None,
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            buffer: None,
            pixel_buffer: Vec::new(),
        }
    }
    fn rasterize_text(&mut self) {
        let width = self.width;
        let height = self.height;

        let required_size = (width * height * 4) as usize;

        if self.pixel_buffer.len() != required_size {
            self.pixel_buffer.resize(required_size, 0);
        }
        self.pixel_buffer.fill(0);

        let Some(buffer) = &mut self.buffer else {
            return;
        };

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, run.line_y), 1.0);

                let Some(image) = self
                    .swash_cache
                    .get_image(&mut self.font_system, physical_glyph.cache_key)
                else {
                    continue;
                };

                let glyph_x = physical_glyph.x + image.placement.left;
                let glyph_y = physical_glyph.y - image.placement.top;

                let glyph_width = image.placement.width as usize;
                let glyph_height = image.placement.height as usize;

                if glyph_width == 0 || glyph_height == 0 {
                    continue;
                }

                for y in 0..glyph_height {
                    for x in 0..glyph_width {
                        let src_index = y * glyph_width + x;

                        let Some(&alpha) = image.data.get(src_index) else {
                            continue;
                        };

                        if alpha == 0 {
                            continue;
                        }

                        let px = glyph_x + x as i32;
                        let py = glyph_y + y as i32;

                        if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                            continue;
                        }

                        let dst_index = (py as usize * width as usize + px as usize) * 4;

                        self.pixel_buffer[dst_index] = 255;
                        self.pixel_buffer[dst_index + 1] = 255;
                        self.pixel_buffer[dst_index + 2] = 255;
                        self.pixel_buffer[dst_index + 3] = alpha;
                    }
                }
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let size = window.inner_size();

        self.width = size.width;
        self.height = size.height;

        let metrics = Metrics::new(24.0, 32.0);

        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        buffer.set_size(Some(self.width as f32), Some(self.height as f32));

        buffer.set_text("Hello, World!", &Attrs::new(), Shaping::Advanced, None);

        buffer.shape_until_scroll(&mut self.font_system, true);

        self.buffer = Some(buffer);

        self.renderer = Some(Renderer::new(window.clone()));
        self.window = Some(window.clone());

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                self.width = size.width;
                self.height = size.height;

                if size.width == 0 || size.height == 0 {
                    return;
                }

                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                if self.width == 0 || self.height == 0 {
                    return;
                }
                self.rasterize_text();

                if let Some(renderer) = &mut self.renderer {
                    renderer.update_texture(&self.pixel_buffer, self.width, self.height);

                    if !renderer.render() {
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                }
            }

            _ => {}
        }
    }
}
