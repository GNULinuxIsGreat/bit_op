#![no_std]

pub mod bit_u128;
pub mod bit_u16;
pub mod bit_u32;
pub mod bit_u64;
pub mod bit_u8;

use core::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not};
pub trait BitOp:
    Not<Output = Self>
    + BitOrAssign<Self>
    + BitAndAssign<Self>
    + BitXorAssign<Self>
    + BitAnd<Self, Output = Self>
    + Sized
{
    fn set(&mut self, rhs: Self) {
        *self |= rhs;
    }

    fn reset(&mut self, rhs: Self) {
        *self &= !rhs;
    }

    fn toggle(&mut self, rhs: Self) {
        *self ^= rhs;
    }

    fn get(self, rhs: Self) -> Self {
        self & rhs
    }
}

impl BitOp for u8 {}
impl BitOp for u16 {}
impl BitOp for u32 {}
impl BitOp for u64 {}
impl BitOp for u128 {}

#[cfg(test)]
mod tests;
