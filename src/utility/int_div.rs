//! Ceil and floor division for positive and negative numbers.
use num::{Integer, Signed};

/// Floor division for positive and negative numbers.
/// This function is equivalent to $`\lfloor x / y \rfloor`$ in mathematics.
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::int_div::div_floor;
/// assert_eq!(div_floor(5, 2), 2);
/// assert_eq!(div_floor(5, -2), -3);
/// assert_eq!(div_floor(-5, 2), -3);
/// assert_eq!(div_floor(-5, -2), 2);
/// ```
///
/// # Panics
/// Panics if `y` is zero.
///
/// # Complexity
/// O(1)
///
/// # Note
/// This function is equivalent to i64::div_floor in nightly.
/// See also: <https://doc.rust-lang.org/std/primitive.i64.html#method.div_floor>
pub fn div_floor<T: Signed + Integer + Clone>(x: T, y: T) -> T {
    assert!(!y.is_zero());
    if y.is_negative() {
        div_floor(-x, -y)
    } else {
        if x.is_positive() {
            x / y
        } else {
            (x - y.clone() + T::one()) / y
        }
    }
}

/// Ceil division for positive and negative numbers.
/// This function is equivalent to $`\lceil x / y \rceil`$ in mathematics.
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::int_div::div_ceil;
/// assert_eq!(div_ceil(5, 2), 3);
/// assert_eq!(div_ceil(5, -2), -2);
/// assert_eq!(div_ceil(-5, 2), -2);
/// assert_eq!(div_ceil(-5, -2), 3);
/// ```
///
/// # Panics
/// Panics if `y` is zero.
///
/// # Complexity
/// O(1)
///
/// # Note
/// This function is equivalent to i64::div_floor in nightly.
/// See also: <https://doc.rust-lang.org/std/primitive.i64.html#method.div_ceil>
pub fn div_ceil<T: Signed + Integer + Clone>(x: T, y: T) -> T {
    assert!(!y.is_zero());
    if y.is_negative() {
        div_ceil(-x, -y)
    } else {
        if x.is_positive() {
            (x + y.clone() - T::one()) / y
        } else {
            x / y
        }
    }
}
