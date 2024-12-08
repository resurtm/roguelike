use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    roguelike::main_loop::MainLoop::new()?.run();
    Ok(())
}
