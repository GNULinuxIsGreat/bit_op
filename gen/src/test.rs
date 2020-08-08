use std::fs::{File, OpenOptions};
use std::io::{Result, Write};
use std::path::Path;

fn set_x(file: &mut File, max: u8) -> Result<()> {
    writeln!(file, "#[test]")?;
    writeln!(file, "fn set_u{}() {{", max)?;
    writeln!(file, "use crate::{{bit_u{}::*, BitOp as _}};", max)?;
    writeln!(file, "    let mut x = 0u{};", max)?;
    for i in 0..max {
        write!(file, "    ")?;
        writeln!(file, "x = 0;")?;
        write!(file, "    ")?;
        writeln!(file, "x.set(B{});", i)?;
        write!(file, "    ")?;
        writeln!(file, "assert!(B{} == x);", i)?;
        write!(file, "    ")?;
        writeln!(file, "x.set(B{});", i)?;
        write!(file, "    ")?;
        writeln!(file, "assert!(B{} == x);", i)?;
    }
    writeln!(file, "}}")?;
    writeln!(file, "");
    Ok(())
}

pub(crate) fn generate() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/tests.rs"))?;
    
    set_x(&mut file, 128)?;
    set_x(&mut file, 64)?;
    set_x(&mut file, 32)?;
    set_x(&mut file, 16)?;
    set_x(&mut file, 8)?;

    Ok(())
}
