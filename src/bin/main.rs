use anyhow::Result;
use clap::Parser;
use splat_lib as lib;
use winit::event_loop::EventLoop;

#[derive(Parser)]
#[command(name = "splat", about = "Open an empty window via wgpu and winit")]
pub struct Cli {
    #[arg(short, long, default_value = "Splat App")]
    pub title: String,

    #[arg(long, default_value_t = 800)]
    pub width: u32,

    #[arg(long, default_value_t = 600)]
    pub height: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let event_loop = EventLoop::new()?;

    let mut app = lib::App::new(cli.title, cli.width, cli.height);

    event_loop.run_app(&mut app)?;

    Ok(())
}
