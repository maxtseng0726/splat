use anyhow::Result;
use clap::Parser;
use winit::event_loop::EventLoop;
use splat_lib as lib;
fn main() -> Result<()> {
    let cli = lib::Cli::parse();

    let event_loop = EventLoop::new()?;

    let mut app = lib::App::new(cli.title, cli.width, cli.height);

    event_loop.run_app(&mut app)?;

    Ok(())
}
