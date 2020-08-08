#![no_std]

/// definitions of constants for each bit of u8
pub mod bit_u8;

/// definitions of constants for each bit of u16
pub mod bit_u16;

/// definitions of constants for each bit of u32
pub mod bit_u32;

/// definitions of constants for each bit of u64
pub mod bit_u64;

/// definitions of constants for each bit of u128
pub mod bit_u128;

use core::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not};
/// provided the methods `set`, `reset`, `toggle`, `get`
pub trait BitOp:
    Not<Output = Self>
    + BitOrAssign<Self>
    + BitAndAssign<Self>
    + BitXorAssign<Self>
    + BitAnd<Self, Output = Self>
    + Sized
{
    /**
    # Set bit that are selected
    ```
    use bit_op::BitOp;
    use bit_op::bit_u8::*;
    let mut x = 0b00001111u8;
    x.set(B7);
    assert_eq!(x, 0b10001111);
    let mut y = 0u8;
    y.set(B7 | B0);
    assert_eq!(y, 0b10000001);
    ```
    */
    fn set(&mut self, rhs: Self) {
        *self |= rhs;
    }

        /**
    # Reset bit that are selected
    ```
    use bit_op::BitOp;
    use bit_op::bit_u8::*;
    let mut x = 0b00001111u8;
    x.reset(B0 | B1 | B2 | B3);
    assert_eq!(x, 0);
    let mut y = 0b11111111u8;
    y.reset(B7 | B0);
    assert_eq!(y, 0b01111110);
    ```
    */
    fn reset(&mut self, rhs: Self) {
        *self &= !rhs;
    }

    /**
    # Toggle bit that are selected
    ```
    use bit_op::BitOp;
    use bit_op::bit_u8::*;
    let mut x = 0b00001111u8;
    x.toggle(B5 | B4 | B3 | B2);
    assert_eq!(x, 0b00110011);
    x.toggle(B5 | B4 | B3 | B2);
    assert_eq!(x, 0b00001111);
    ```
    */
    fn toggle(&mut self, rhs: Self) {
        *self ^= rhs;
    }

        /**
    # Get the bits that are desired
    ```
    use bit_op::BitOp;
    use bit_op::bit_u8::*;
    let mut x = 0b10000001u8;
    assert_eq!(x.get(B7), 0b10000000);
    assert_eq!(x.get(B6), 0b00000000);
    assert_eq!(x.get(B0), 0b00000001);
    ```
    */
    fn get(self, rhs: Self) -> Self {
        self & rhs
    }
}

impl BitOp for u8 {}
impl BitOp for u16 {}
impl BitOp for u32 {}
impl BitOp for u64 {}
impl BitOp for u128 {}
