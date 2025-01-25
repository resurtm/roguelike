/// Main application entry point.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(roguelike::app::launch())?;
    Ok(())
}
