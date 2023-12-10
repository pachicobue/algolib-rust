//! Modular operations

/// Returns $`x + y \pmod m`$
///
/// # Constraints
///
/// - $`0 \leq x`$
/// - $`0 \leq y`$
/// - $`1 \leq m`$
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::mod_ops::mod_add;
///
/// assert_eq!(mod_add(1, 2, 3), 0);
/// assert_eq!(mod_add(3, 6, 7), 2);
/// assert_eq!(mod_add(1_000_000_000, 1_000_000_000, 1_000_000_007), 999999993);
/// ```
///
/// # Complexity
///
/// $`O(1)`$
pub fn mod_add(x: u64, y: u64, m: u64) -> u64 {
    assert_ne!(m, 0);
    (x + y) % m
}

/// Returns $`x - y \pmod m`$
///
/// # Constraints
///
/// - $`0 \leq x`$
/// - $`0 \leq y`$
/// - $`1 \leq m`$
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::mod_ops::mod_sub;
///
/// assert_eq!(mod_sub(1, 2, 3), 2);
/// assert_eq!(mod_sub(3, 6, 7), 4);
/// assert_eq!(mod_sub(1_000_000_000, 1_000_000_000, 1_000_000_007), 0);
/// ```
///
/// # Complexity
///
/// $`O(1)`$
pub fn mod_sub(x: u64, y: u64, m: u64) -> u64 {
    assert_ne!(m, 0);
    mod_add(x, m - (y % m), m)
}

/// Returns $`x \times y \pmod m`$
///
/// # Constraints
///
/// - $`0 \leq x`$
/// - $`0 \leq y`$
/// - $`1 \leq m`$
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::mod_ops::mod_mul;
///
/// assert_eq!(mod_mul(1, 2, 3), 2);
/// assert_eq!(mod_mul(3, 6, 7), 4);
/// assert_eq!(mod_mul(1_000_000_000, 1_000_000_000, 1_000_000_007), 49);
/// ```
///
/// # Complexity
///
/// $`O(1)`$
pub fn mod_mul(x: u64, y: u64, m: u64) -> u64 {
    assert_ne!(m, 0);
    x * y % m
}

/// Returns $`x^n \pmod m`$
///
/// # Constraints
///
/// - $`0 \leq x`$
/// - $`0 \leq n`$
/// - $`1 \leq m`$
///
/// # Examples
///
/// ```
/// use algolib_rust::utility::mod_ops::mod_pow;
///
/// assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
/// assert_eq!(mod_pow(2, 1_000_000_000, 1_000_000_007), 140625001);
/// assert_eq!(mod_pow(1_000_000_000, 1_000_000_000, 1_000_000_007), 312556845);
/// assert_eq!(mod_pow(1_000_000_000, 1_000_000_000_000_000_000, 998244353u64), 923406214);
/// ```
///
/// # Complexity
///
/// $`O(\log n)`$
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64
where
    U: Unsigned + Integer + Clone,
    U2: Unsigned + Integer,
{
    assert!(!m.is_zero());
    let two: U2 = U2::one() + U2::one();
    if n.is_zero() {
        U::one() % m
    } else if n.is_even() {
        mod_pow(mod_mul(x.clone(), x, m.clone()), n / two, m)
    } else {
        mod_mul(x.clone(), mod_pow(x, n - U2::one(), m.clone()), m)
    }
}
