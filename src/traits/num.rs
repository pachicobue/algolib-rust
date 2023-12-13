//! 数値に関するトレイト
use std::ops::{Add, Mul, Neg};

/// 加算の単位元0を持つことを表すトレイト
pub trait Zero: Add<Output = Self> + Sized {
    fn zero() -> Self;
}

/// 乗算の単位元1を持つことを表すトレイト
pub trait One: Mul<Output = Self> + Sized {
    fn one() -> Self;
}

/// 符号付き数値に関するトレイト
pub trait Signed: Neg<Output = Self> + PartialOrd + Zero {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

/// 符号なし数値に関するトレイト
pub trait Unsigned {}

/// 有界な数値に関するトレイト
pub trait Bounded: PartialOrd {
    fn min_value() -> Self;
    fn max_value() -> Self;
}
