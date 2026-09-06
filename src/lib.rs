use clap::Parser;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
#[derive(Parser)]
#[command(name = "splat", about = "Open an empty window via winit")]
pub struct Cli {
    /// Window title
    #[arg(short, long, default_value = "Splat App")]
    pub title: String,

    /// Window width
    #[arg(long, default_value_t = 800)]
    pub width: u32,

    /// Window height
    #[arg(long, default_value_t = 600)]
    pub height: u32,
}

pub struct App {
    title: String,
    width: u32,
    height: u32,
    window: Option<Window>,
}

impl App {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            window: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = event_loop.create_window(attrs).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}
