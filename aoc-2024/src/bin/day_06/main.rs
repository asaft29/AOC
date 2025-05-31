use anyhow::Result;
mod p1;
mod p2;
fn main() -> Result<()> {
    p1::exec()?;
    p2::exec()?;
    Ok(())
}
