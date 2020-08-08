use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::path::Path;

fn generate_u8() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/bit_u8.rs"))?;

    for i in 0..=7 {
        writeln!(
            file,
            "pub const B{}: u8 = crate::bit_u128::B{} as u8;",
            i, i
        )?;
    }
    Ok(())
}

fn generate_u16() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/bit_u16.rs"))?;

    for i in 0..=15 {
        writeln!(
            file,
            "pub const B{}: u16 = crate::bit_u128::B{} as u16;",
            i, i
        )?;
    }
    Ok(())
}

fn generate_u32() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/bit_u32.rs"))?;

    for i in 0..=31 {
        writeln!(
            file,
            "pub const B{}: u32 = crate::bit_u128::B{} as u32;",
            i, i
        )?;
    }

    Ok(())
}

fn generate_u64() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/bit_u64.rs"))?;

    for i in 0..=63 {
        writeln!(
            file,
            "pub const B{}: u64 = crate::bit_u128::B{} as u64;",
            i, i
        )?;
    }

    Ok(())
}

fn generate_u128() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new("../src/bit_u128.rs"))?;

    for i in 0..=127 {
        writeln!(file, "pub const B{}: u128 = 1 << {};", i, i)?;
    }

    Ok(())
}

pub(crate) fn generate() -> Result<()> {
    generate_u8()?;
    generate_u16()?;
    generate_u32()?;
    generate_u64()?;
    generate_u128()?;
    Ok(())
}
