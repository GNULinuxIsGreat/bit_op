use super::*;

pub trait U8op:
    Sized
    + core::ops::BitAnd<u8, Output = u8>
    + core::ops::BitOrAssign<u8>
    + core::ops::BitAndAssign<u8>
    + core::ops::BitXorAssign<u8>
{
    /// return bit 0
    #[inline(always)]
    fn b0(self) -> bool {
        (self & bit_u8::B0) != 0
    }

    /// return bit 1
    #[inline(always)]
    fn b1(self) -> bool {
        (self & bit_u8::B1) != 0
    }

    /// return bit 2
    #[inline(always)]
    fn b2(self) -> bool {
        (self & bit_u8::B2) != 0
    }

    /// return bit 3
    #[inline(always)]
    fn b3(self) -> bool {
        (self & bit_u8::B3) != 0
    }

    /// return bit 4
    #[inline(always)]
    fn b4(self) -> bool {
        (self & bit_u8::B4) != 0
    }

    /// return bit 5
    #[inline(always)]
    fn b5(self) -> bool {
        (self & bit_u8::B5) != 0
    }

    /// return bit 6
    #[inline(always)]
    fn b6(self) -> bool {
        (self & bit_u8::B6) != 0
    }

    /// return bit 7
    #[inline(always)]
    fn b7(self) -> bool {
        (self & bit_u8::B7) != 0
    }

    /// set bit 0
    #[inline(always)]
    fn set_b0(&mut self) {
        *self |= bit_u8::B0;
    }

    /// set bit 1
    #[inline(always)]
    fn set_b1(&mut self) {
        *self |= bit_u8::B1;
    }

    /// set bit 2
    #[inline(always)]
    fn set_b2(&mut self) {
        *self |= bit_u8::B2;
    }

    /// set bit 3
    #[inline(always)]
    fn set_b3(&mut self) {
        *self |= bit_u8::B3;
    }

    /// set bit 4
    #[inline(always)]
    fn set_b4(&mut self) {
        *self |= bit_u8::B4;
    }

    /// set bit 5
    #[inline(always)]
    fn set_b5(&mut self) {
        *self |= bit_u8::B5;
    }

    /// set bit 6
    #[inline(always)]
    fn set_b6(&mut self) {
        *self |= bit_u8::B6;
    }

    /// set bit 7
    #[inline(always)]
    fn set_b7(&mut self) {
        *self |= bit_u8::B7;
    }

    /// reset bit 0
    #[inline(always)]
    fn reset_b0(&mut self) {
        *self &= !bit_u8::B0;
    }

    /// reset bit 1
    #[inline(always)]
    fn reset_b1(&mut self) {
        *self &= !bit_u8::B1;
    }

    /// reset bit 2
    #[inline(always)]
    fn reset_b2(&mut self) {
        *self &= !bit_u8::B2;
    }

    /// reset bit 3
    #[inline(always)]
    fn reset_b3(&mut self) {
        *self &= !bit_u8::B3;
    }

    /// reset bit 4
    #[inline(always)]
    fn reset_b4(&mut self) {
        *self &= !bit_u8::B4;
    }

    /// reset bit 5
    #[inline(always)]
    fn reset_b5(&mut self) {
        *self &= !bit_u8::B5;
    }

    /// reset bit 6
    #[inline(always)]
    fn reset_b6(&mut self) {
        *self &= !bit_u8::B6;
    }

    /// reset bit 7
    #[inline(always)]
    fn reset_b7(&mut self) {
        *self &= !bit_u8::B7;
    }

    /// toggle bit 0
    #[inline(always)]
    fn toggle_b0(&mut self) {
        *self ^= bit_u8::B0;
    }

    /// toggle bit 1
    #[inline(always)]
    fn toggle_b1(&mut self) {
        *self ^= bit_u8::B1;
    }

    /// toggle bit 2
    #[inline(always)]
    fn toggle_b2(&mut self) {
        *self ^= bit_u8::B2;
    }

    /// toggle bit 3
    #[inline(always)]
    fn toggle_b3(&mut self) {
        *self ^= bit_u8::B3;
    }

    /// toggle bit 4
    #[inline(always)]
    fn toggle_b4(&mut self) {
        *self ^= bit_u8::B4;
    }

    /// toggle bit 5
    #[inline(always)]
    fn toggle_b5(&mut self) {
        *self ^= bit_u8::B5;
    }

    /// toggle bit 6
    #[inline(always)]
    fn toggle_b6(&mut self) {
        *self ^= bit_u8::B6;
    }

    /// toggle bit 7
    #[inline(always)]
    fn toggle_b7(&mut self) {
        *self ^= bit_u8::B7;
    }
}
