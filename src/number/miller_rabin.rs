//! Miller-Rabin test

use crate::number::mod_ops::mod_pow;

const fn miller_rabin_test(m: u32, a_list: [u32]) -> bool {
    let d = (m - 1) >> (m - 1).trailing_zeros();
    for a in a_list {
        if m <= a {
            break;
        }

        let mut t = mod_pow(a, d, m);
        while d != m - 1 && t != 1 && t != m - 1 {
            t = mod_mul(t, t, m);
            d <<= 1;
        }
        if t != m - 1 && d % 2 == 0 {
            return false;
        }
    }
    true
}

/// Returns whether $`m`$ is a prime number.
///
/// # Constraints
///
/// - $`0 \leq m`$
///
/// # Examples
///
/// ```
/// use algolib_rust::number::miller_rabin::is_prime;
/// assert_eq!(is_prime(0), false);
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(3), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(5), true);
/// assert_eq!(is_prime(998244353), true);
/// assert_eq!(is_prime(1000000007), true);
/// assert_eq!(is_prime(1000000009), true);
/// assert_eq!(is_prime(1000000011), false);
/// ```
///
/// # Complexity
///
/// $`O((\log m)^2)`$
pub const fn is_prime(m: u32) -> bool {
    if m <= 1 {
        false
    } else if m == 2 || m == 7 || m == 61 {
        true
    } else {
        miller_rabin_test(m, [2, 7, 61])
    }
}
