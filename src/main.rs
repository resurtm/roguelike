fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(roguelike::window::run())?;
    // roguelike::main_loop::MainLoop::new()?.run()?;
    Ok(())
}
