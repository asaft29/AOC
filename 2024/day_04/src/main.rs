mod part_1;
mod part_2;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    part_1::execute()?;
    part_2::execute()?;
    Ok(())
}
