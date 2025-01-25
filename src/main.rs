fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(roguelike::app::launch())?;
    // roguelike::main_loop::MainLoop::new()?.run()?;
    Ok(())
}
