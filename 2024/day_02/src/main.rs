use std::error::Error;

mod part_1;
fn main() -> Result<(), Box<dyn Error>>{
    part_1::execute()?;
    Ok(())
}