mod part_1;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    part_1::execute()?;
    Ok(())
}
