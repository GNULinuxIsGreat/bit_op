mod constant;
use std::io::Result;

fn main() -> Result<()> {
    constant::generate()?;
    Ok(())
}
