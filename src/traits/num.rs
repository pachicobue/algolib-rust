/// 基本的な数値型のトレイト
use ::std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

mod bounded;
mod signed;
mod zero_one;

pub use bounded::*;
pub use signed::*;
pub use zero_one::*;

/// (0,1,+,*,/,%,+=,-=,*=,/=,%=) が使える数値型
pub trait Num:
    PartialEq
    + Zero
    + One
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    + Add<&Self, Output = Self>
    + Sub<&Self, Output = Self>
    + Mul<&Self, Output = Self>
    + Div<&Self, Output = Self>
    + Rem<&Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
{
}

impl<T> Num for T where
    T: PartialEq
        + Zero
        + One
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Rem<Self, Output = Self>
        + AddAssign<Self>
        + SubAssign<Self>
        + MulAssign<Self>
        + DivAssign<Self>
        + RemAssign<Self>
{
}

pub trait Integer: Num + Ord + Eq {}
pub trait Real: Num + PartialOrd {
    fn nan() -> Self;
    fn is_nan(&self) -> bool;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_infinity(&self) -> bool;
    fn is_finite(&self) -> bool;

    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
}
