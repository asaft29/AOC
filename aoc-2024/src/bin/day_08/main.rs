use anyhow::Result;
mod p1;

fn main() -> Result<()> {
    p1::execute()?;
    Ok(())
}
