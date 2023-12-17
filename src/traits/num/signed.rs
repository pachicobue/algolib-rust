use super::zero_one::Zero;
use std::ops::Neg;

/// 符号付き数値に関するトレイト
pub trait Signed: Neg<Output = Self> + PartialOrd + Zero {
    /// 絶対値を返す
    fn abs(&self) -> Self;
    /// 正の数値かどうかを返す
    fn is_positive(&self) -> bool;
    /// 負の数値かどうかを返す
    fn is_negative(&self) -> bool;
}
macro_rules! impl_signed {
    ($t:ty) => {
        impl Signed for $t {
            fn abs(&self) -> Self {
                <$t>::abs(*self)
            }
            fn is_positive(&self) -> bool {
                <$t>::is_positive(*self)
            }
            fn is_negative(&self) -> bool {
                <$t>::is_negative(*self)
            }
        }
    };
}
impl_signed!(isize);
impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
#[cfg(has_i128)]
impl_signed!(i128);

macro_rules! impl_signed_float {
    ($t:ty) => {
        impl Signed for $t {
            fn abs(&self) -> Self {
                <$t>::abs(*self)
            }
            fn is_positive(&self) -> bool {
                <$t>::is_sign_positive(*self)
            }
            fn is_negative(&self) -> bool {
                <$t>::is_sign_negative(*self)
            }
        }
    };
}
impl_signed_float!(f32);
impl_signed_float!(f64);

/// 符号なし数値に関するトレイト
pub trait Unsigned {}

macro_rules! impl_unsigned {
    ($t:ty) => {
        impl Unsigned for $t {}
    };
}
impl_unsigned!(usize);
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
#[cfg(has_i128)]
impl_unsigned!(u128);

#[cfg(test)]
mod tests {
    #[test]
    fn test_abs() {
        let a: i32 = -2;
        assert_eq!(a.abs(), 2);

        let a: f32 = -0.8;
        assert_eq!(a.abs(), 0.8);
    }
}
