use std::ops::{Add, Mul};

/// 加算の単位元0を持つことを表すトレイト
pub trait Zero: Add<Self> + Sized + PartialEq {
    /// 加算の単位元0を返す
    fn zero() -> Self;
    /// 0かどうかを返す
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> $t {
                $v
            }
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

impl_zero!(usize, 0);
impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
#[cfg(has_i128)]
impl_zero!(u128, 0);

impl_zero!(isize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
#[cfg(has_i128)]
impl_zero!(i128, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

pub trait One: Mul<Output = Self> + Sized {
    /// 乗算の単位元1を返す
    fn one() -> Self;
    /// 1かどうかを返す
    fn is_one(&self) -> bool;
}

macro_rules! impl_one {
    ($t:ty, $v:expr) => {
        impl One for $t {
            fn one() -> $t {
                $v
            }
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    };
}
impl_one!(usize, 1);
impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
#[cfg(has_i128)]
impl_one!(u128, 1);

impl_one!(isize, 1);
impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
#[cfg(has_i128)]
impl_one!(i128, 1);

impl_one!(f32, 1.0);
impl_one!(f64, 1.0);
