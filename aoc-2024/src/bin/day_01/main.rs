use std::error::Error;

mod p1;
mod p2;
fn main() -> Result<(), Box<dyn Error>> {
    p1::execute()?;
    p2::execute()?;
    Ok(())
}
