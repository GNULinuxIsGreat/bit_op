mod constant;
mod test;
use std::io::Result;

fn main() -> Result<()> {
    constant::generate()?;
    test::generate()?;
    Ok(())
}
