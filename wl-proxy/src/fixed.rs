//! A signed 24.8 fixed-point number used in the wayland protocol.

#[cfg(test)]
mod tests;

use std::{
    fmt::{Debug, Display, Formatter},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

/// A signed 24.8 fixed-point number used in the wayland protocol.
///
/// This is a signed decimal type which offers a sign bit, 23 bits of integer precision and 8 bits
/// of decimal precision.
///
/// # Arithmetic operations
///
/// This type implements all of the usual arithmetic operations for numbers. On overflow, they
/// behave like the standard library operations except that multiplication and division always use
/// wrapping semantics.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Fixed(i32);

macro_rules! fmul {
    () => {
        256.0
    };
}
macro_rules! imul {
    () => {
        256
    };
}
macro_rules! shift {
    () => {
        8
    };
}

impl Fixed {
    /// The largest [`Fixed`].
    pub const MAX: Self = Self(i32::MAX);

    /// The smallest [`Fixed`].
    pub const MIN: Self = Self(i32::MIN);

    /// The 0 [`Fixed`].
    pub const ZERO: Self = Self(0);

    /// The 1 [`Fixed`].
    pub const ONE: Self = Self::from_i32_saturating(1);

    /// The 2 [`Fixed`].
    pub const TWO: Self = Self::from_i32_saturating(2);

    /// The smallest positive [`Fixed`].
    pub const EPSILON: Self = Self(1);

    /// The largest negative [`Fixed`].
    pub const NEGATIVE_EPSILON: Self = Self(!0);

    /// Creates a [`Fixed`] from the raw bits that appear in the wire protocol.
    #[inline]
    pub const fn from_wire(val: i32) -> Self {
        Self(val)
    }

    /// Converts this [`Fixed`] to the bits that should be set in the wire protocol.
    #[inline]
    pub const fn to_wire(self) -> i32 {
        self.0
    }

    /// Converts this [`Fixed`] to an `f64`.
    ///
    /// This conversion is lossless.
    #[inline]
    pub const fn to_f64(self) -> f64 {
        self.0 as f64 / fmul!()
    }

    /// Converts this [`Fixed`] to an `f32`.
    ///
    /// This conversion is lossy if there are more than 24 significant bits in this [`Fixed`].
    #[inline]
    pub const fn to_f32_lossy(self) -> f32 {
        self.to_f64() as f32
    }

    /// Creates a [`Fixed`] from an `f64`.
    ///
    /// If the value cannot be represented exactly, the behavior is as when an `f64` is cast to an
    /// integer. That is
    ///
    /// - Values are rounded towards 0.
    /// - `NaN` returns [`Fixed::ZERO`].
    /// - Values larger than the maximum return [`Fixed::MAX`].
    /// - Values smaller than the minimum return [`Fixed::MIN`].
    #[inline]
    pub const fn from_f64_lossy(val: f64) -> Self {
        Self((val * fmul!()) as i32)
    }

    /// Creates a [`Fixed`] from an `f32`.
    ///
    /// The conversion behavior is the same as for [`Fixed::from_f64_lossy`].
    #[inline]
    pub const fn from_f32_lossy(val: f32) -> Self {
        Self((val as f64 * fmul!()) as i32)
    }

    /// Creates a [`Fixed`] from an `i32`.
    ///
    /// Values outside of the representable range are clamped to [`Fixed::MIN`] and [`Fixed::MAX`].
    #[inline]
    pub const fn from_i32_saturating(val: i32) -> Self {
        Self(val.saturating_mul(imul!()))
    }

    /// Creates a [`Fixed`] from an `i64`.
    ///
    /// Values outside of the representable range are clamped to [`Fixed::MIN`] and [`Fixed::MAX`].
    #[inline]
    pub const fn from_i64_saturating(val: i64) -> Self {
        let val = val.saturating_mul(imul!());
        if val > i32::MAX as i64 {
            Self(i32::MAX)
        } else if val < i32::MIN as i64 {
            Self(i32::MIN)
        } else {
            Self(val as i32)
        }
    }

    /// Converts this [`Fixed`] to an `i32`.
    ///
    /// The conversion rounds towards the nearest integer and half-way away from 0.
    #[inline]
    pub const fn to_i32_round_towards_nearest(self) -> i32 {
        if self.0 >= 0 {
            ((self.0 as i64 + (imul!() / 2)) / imul!()) as i32
        } else {
            ((self.0 as i64 - (imul!() / 2)) / imul!()) as i32
        }
    }

    /// Converts this [`Fixed`] to an `i32`.
    ///
    /// The conversion rounds towards zero.
    #[inline]
    pub const fn to_i32_round_towards_zero(self) -> i32 {
        (self.0 as i64 / imul!()) as i32
    }

    /// Converts this [`Fixed`] to an `i32`.
    ///
    /// The conversion rounds towards minus infinity.
    #[inline]
    pub const fn to_i32_floor(self) -> i32 {
        self.0 >> shift!()
    }

    /// Converts this [`Fixed`] to an `i32`.
    ///
    /// The conversion rounds towards infinity.
    #[inline]
    pub const fn to_i32_ceil(self) -> i32 {
        ((self.0 as i64 + imul!() - 1) >> shift!()) as i32
    }
}

macro_rules! from {
    ($t:ty) => {
        impl From<$t> for Fixed {
            #[inline]
            fn from(value: $t) -> Self {
                Self(value as i32 * imul!())
            }
        }
    };
}

from!(i8);
from!(u8);
from!(i16);
from!(u16);

impl From<Fixed> for f64 {
    #[inline]
    fn from(value: Fixed) -> Self {
        value.to_f64()
    }
}

impl Debug for Fixed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.to_f64(), f)
    }
}

impl Display for Fixed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_f64(), f)
    }
}

macro_rules! forward_simple_immutable_binop {
    ($slf:ty, $arg:ty, $big_name:ident, $small_name:ident, $op:tt) => {
        impl $big_name<$arg> for $slf {
            type Output = Fixed;

            #[inline]
            fn $small_name(self, rhs: $arg) -> Self::Output {
                Fixed(self.0 $op rhs.0)
            }
        }
    };
}

macro_rules! forward_simple_binop {
    ($big_name:ident, $small_name:ident, $op:tt, $assign_big_name:ident, $assign_small_name:ident, $assign_op:tt) => {
        forward_simple_immutable_binop!(Fixed,  Fixed,  $big_name, $small_name, $op);
        forward_simple_immutable_binop!(Fixed,  &Fixed, $big_name, $small_name, $op);
        forward_simple_immutable_binop!(&Fixed, Fixed,  $big_name, $small_name, $op);
        forward_simple_immutable_binop!(&Fixed, &Fixed, $big_name, $small_name, $op);

        impl $assign_big_name for Fixed {
            #[inline]
            fn $assign_small_name(&mut self, rhs: Self) {
                self.0 $assign_op rhs.0;
            }
        }
    };
}

forward_simple_binop!(Add,    add,    +, AddAssign,    add_assign,    +=);
forward_simple_binop!(Sub,    sub,    -, SubAssign,    sub_assign,    -=);
forward_simple_binop!(Rem,    rem,    %, RemAssign,    rem_assign,    %=);
forward_simple_binop!(BitAnd, bitand, &, BitAndAssign, bitand_assign, &=);
forward_simple_binop!(BitOr,  bitor,  |, BitOrAssign,  bitor_assign,  |=);
forward_simple_binop!(BitXor, bitxor, ^, BitXorAssign, bitxor_assign, ^=);

#[inline(always)]
const fn mul(slf: i32, rhs: i32) -> i32 {
    (slf as i64 * rhs as i64 / imul!()) as i32
}

#[inline(always)]
const fn div(slf: i32, rhs: i32) -> i32 {
    (slf as i64 * imul!() / rhs as i64) as i32
}

macro_rules! forward_complex_immutable_binop {
    ($slf:ty, $arg:ty, $big_name:ident, $small_name:ident) => {
        impl $big_name<$arg> for $slf {
            type Output = Fixed;

            #[inline]
            fn $small_name(self, rhs: $arg) -> Self::Output {
                Fixed($small_name(self.0, rhs.0))
            }
        }
    };
}

macro_rules! forward_complex_binop {
    ($big_name:ident, $small_name:ident, $assign_big_name:ident, $assign_small_name:ident) => {
        forward_complex_immutable_binop!(Fixed, Fixed, $big_name, $small_name);
        forward_complex_immutable_binop!(Fixed, &Fixed, $big_name, $small_name);
        forward_complex_immutable_binop!(&Fixed, Fixed, $big_name, $small_name);
        forward_complex_immutable_binop!(&Fixed, &Fixed, $big_name, $small_name);

        impl $assign_big_name for Fixed {
            #[inline]
            fn $assign_small_name(&mut self, rhs: Self) {
                self.0 = $small_name(self.0, rhs.0);
            }
        }
    };
}

forward_complex_binop!(Mul, mul, MulAssign, mul_assign);
forward_complex_binop!(Div, div, DivAssign, div_assign);

macro_rules! forward_shiftop {
    ($big_name:ident, $small_name:ident, $arg:ty, $op:tt, $assign_big_name:ident, $assign_small_name:ident, $assign_op:tt) => {
        impl $big_name<$arg> for Fixed {
            type Output = Fixed;

            #[inline]
            fn $small_name(self, rhs: $arg) -> Self::Output {
                Fixed(self.0 $op rhs)
            }
        }

        impl $big_name<$arg> for &Fixed {
            type Output = Fixed;

            #[inline]
            fn $small_name(self, rhs: $arg) -> Self::Output {
                Fixed(self.0 $op rhs)
            }
        }

        impl $assign_big_name<$arg> for Fixed {
            #[inline]
            fn $assign_small_name(&mut self, rhs: $arg) {
                self.0 $assign_op rhs;
            }
        }
    };
}

macro_rules! forward_shift {
    ($arg:ty) => {
        forward_shiftop!(Shl, shl, $arg,  <<, ShlAssign, shl_assign, <<=);
        forward_shiftop!(Shl, shl, &$arg, <<, ShlAssign, shl_assign, <<=);
        forward_shiftop!(Shr, shr, $arg,  >>, ShrAssign, shr_assign, >>=);
        forward_shiftop!(Shr, shr, &$arg, >>, ShrAssign, shr_assign, >>=);
    }
}

forward_shift!(u8);
forward_shift!(i8);
forward_shift!(u16);
forward_shift!(i16);
forward_shift!(u32);
forward_shift!(i32);
forward_shift!(u64);
forward_shift!(i64);
forward_shift!(u128);
forward_shift!(i128);
forward_shift!(usize);
forward_shift!(isize);

macro_rules! forward_immutable_unop {
    ($slf:ty, $big_name:ident, $small_name:ident, $op:tt) => {
        impl $big_name for $slf {
            type Output = Fixed;

            #[inline]
            fn $small_name(self) -> Self::Output {
                Fixed($op self.0)
            }
        }
    };
}

macro_rules! forward_unop {
    ($big_name:ident, $small_name:ident, $op:tt) => {
        forward_immutable_unop!(Fixed, $big_name, $small_name, $op);
        forward_immutable_unop!(&Fixed, $big_name, $small_name, $op);
    };
}

forward_unop!(Neg, neg, -);
forward_unop!(Not, not, !);
