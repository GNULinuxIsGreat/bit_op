use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use std::io::Result;

fn generate_u8() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit_u8.rs"))?;
    
    for i in 0..=7 {
        writeln!(file, "pub const B{}: u8 = 1 << {};", i, i)?;
    }
    Ok(())
}

fn generate_u16() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit_u16.rs"))?;
    
    for i in 0..=7 {
        writeln!(file, "pub const B{}: u16 = crate::bit::B{} as u16;", i, i)?;
    }

    for i in 8..=15 {
        writeln!(file, "pub const B{}: u16 = 1 << {};", i, i)?;
    }
    Ok(())
}

fn generate_u32() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit_u32.rs"))?;
    
    for i in 0..=15 {
        writeln!(file, "pub const B{}: u32 = crate::bit::B{} as u32;", i, i)?;
    }

    for i in 16..=31 {
        writeln!(file, "pub const B{}: u32 = 1 << {};", i, i)?;
    }

    Ok(())
}

fn generate_u64() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit_u64.rs"))?;
    
    for i in 0..=31 {
        writeln!(file, "pub const B{}: u64 = crate::bit::B{} as u64;", i, i)?;
    }

    for i in 32..=63 {
        writeln!(file, "pub const B{}: u64 = 1 << {};", i, i)?;
    }

    Ok(())
}

fn generate_u128() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit_u128.rs"))?;
    
    for i in 0..=63 {
        writeln!(file, "pub const B{}: u128 = crate::bit::B{} as u128;", i, i)?;
    }

    for i in 64..=127 {
        writeln!(file, "pub const B{}: u128 = 1 << {};", i, i)?;
    }

    Ok(())
}

fn generate_bit() -> Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(Path::new("../src/bit.rs"))?;
    
    for i in 0..=7 {
        writeln!(file, "pub use crate::bit_u8::B{};", i)?;
    }

    for i in 8..=15 {
        writeln!(file, "pub use crate::bit_u16::B{};", i)?;
    }
    
    for i in 16..=31 {
        writeln!(file, "pub use crate::bit_u32::B{};", i)?;
    }

    for i in 32..=63 {
        writeln!(file, "pub use crate::bit_u64::B{};", i)?;
    }

    for i in 64..=127 {
        writeln!(file, "pub use crate::bit_u128::B{};", i)?;
    }

    Ok(())
}

pub (crate) fn generate() -> Result<()> {
    generate_u8()?;
    generate_u16()?;
    generate_u32()?;
    generate_u64()?;
    generate_u128()?;
    generate_bit()?;
    Ok(())
}