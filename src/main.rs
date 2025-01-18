fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(roguelike::window::run())?;
    Ok(())
}
