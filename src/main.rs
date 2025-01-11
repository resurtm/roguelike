fn main() -> Result<(), Box<dyn std::error::Error>> {
    roguelike::app::run();
    // roguelike::main_loop::MainLoop::new()?.run()?;
    Ok(())
}
