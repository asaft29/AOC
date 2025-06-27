use anyhow::Result;
mod p1;
mod p2;

fn main() -> Result<()> {
    p1::execute()?;
    p2::execute()?;
    Ok(())
}
