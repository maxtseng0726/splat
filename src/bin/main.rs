use clap::Parser;
use splat_lib as lib;
use winit::event_loop::EventLoop;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = lib::Cli::parse();

    let event_loop = EventLoop::new()?;

    let mut app = lib::App::new(cli.title, cli.width, cli.height);

    event_loop.run_app(&mut app)?;

    Ok(())
}
