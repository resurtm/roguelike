fn main() -> Result<(), Box<dyn std::error::Error>> {
    roguelike::main_loop::MainLoop::new()?.run();
    Ok(())
}
