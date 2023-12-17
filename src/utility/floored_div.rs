//! 切り捨て除算、切り上げ除算を実装するためのトレイト
use crate::traits::num::Integer;
use crate::traits::num::Signed;

pub trait FlooredDiv: Integer {
    /// $`\lfloor x / y \rfloor`$を返す
    fn div_floor(&self, y: &Self) -> Self {
        *self / *y
    }
    /// $`\lceil x / y \rceil`$を返す
    fn div_ceil(&self, y: &Self) -> Self {
        let (d, r) = (*self / *y, *self % *y);
        if r.is_zero() {
            d
        } else {
            d + Self::one()
        }
    }
}

// (Rust仕様) x%y の符号は x と同じ
// x    y      d      r    | divFloor divCeil
// ------------------------------------------
// 3  = 2    * 1    + 1    | 1        2
// 3  = (-2) * (-1) + 1    | -2       -1
// -3 = 2    * (-1) + (-1) | -2       -1
// -3 = (-2) * 1    + (-1) | 1        2
impl<T: Integer + Signed> FlooredDiv for T {
    fn div_floor(&self, y: &Self) -> Self {
        let (d, r) = (*self / *y, *self % *y);
        if (r.is_positive() && y.is_negative()) || (r.is_negative() && y.is_positive()) {
            d - Self::one()
        } else {
            d
        }
    }
    fn div_ceil(&self, y: &Self) -> Self {
        let (d, r) = (*self / *y, *self % *y);
        if (r.is_positive() && y.is_positive()) || (r.is_negative() && y.is_negative()) {
            d + Self::one()
        } else {
            d
        }
    }
}
