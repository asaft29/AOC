mod p1;
mod p2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    p1::execute()?;
    p2::execute()?;
    Ok(())
}
