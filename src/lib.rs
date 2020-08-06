#![no_std]

pub mod bit;
pub mod bit_u128;
pub mod bit_u16;
pub mod bit_u32;
pub mod bit_u64;
pub mod bit_u8;

pub mod bit_op;

/*pub trait B0B15:
    Sized
    + core::ops::BitAnd<u8, Output = Self>
    + core::ops::BitOrAssign<u8>
    + core::ops::BitAndAssign<u8>
    + core::ops::BitXorAssign<u8>
{
}*/

/*pub trait B0B31:
    Sized
    + core::ops::BitAnd<u8, Output = Self>
    + core::ops::BitOrAssign<u8>
    + core::ops::BitAndAssign<u8>
    + core::ops::BitXorAssign<u8>
{
}

pub trait B0B63:
    Sized
    + core::ops::BitAnd<u8, Output = Self>
    + core::ops::BitOrAssign<u8>
    + core::ops::BitAndAssign<u8>
    + core::ops::BitXorAssign<u8>
{
}

pub trait B0B127:
    Sized
    + core::ops::BitAnd<u8, Output = Self>
    + core::ops::BitOrAssign<u8>
    + core::ops::BitAndAssign<u8>
    + core::ops::BitXorAssign<u8>
{
}*/

//impl<T: B0B15> B0B7 for T {}
//impl<T: B0B31> B0B15 for T {}
//impl<T: B0B63> B0B31 for T {}
//impl<T: B0B127> B0B63 for T {}

impl bit_op::U8op for u8 {}
//impl B0B15 for u8 {}

#[cfg(test)]
mod tests;
