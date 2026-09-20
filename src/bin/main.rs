use std::path::PathBuf;

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

    #[arg(short, long)]
    pub source: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let text = match cli.source {
        Some(path) => std::fs::read_to_string(path)?,
        None => "Hello, World!".to_string(),
    };

    let event_loop = EventLoop::new()?;

    let mut app = lib::App::new(cli.title, cli.width, cli.height, text);

    event_loop.run_app(&mut app)?;

    Ok(())
}
