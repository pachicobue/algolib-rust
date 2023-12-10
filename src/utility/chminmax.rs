//! Update the value of the first argument to the minimum/maximum of the two arguments.

/// Update the value of the first argument to the minimum of the two arguments.
/// Returns true if the value is updated.
///
/// # Examples
///
/// ```
/// use algolib_rust::chmin;
///
/// let mut a = 2;
/// let b = 1;
/// assert!(chmin!(a, b));
/// assert_eq!(a, 1);
///
/// let mut a = 1;
/// let b = 3;
/// assert!(!chmin!(a, b));
/// assert_eq!(a, 1);
///
/// let mut v = vec![3, 1];
/// assert!(chmin!(v[0], v[1]));
/// assert_eq!(v[0], 1);
///
/// let mut v = vec![3, 1];
/// assert!(!chmin!(v[1], v[0]));
/// assert_eq!(v[1], 1);
/// ```
#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {{
        let b_val = $b.clone();
        let min = std::cmp::min($a, b_val);
        if $a > min {
            $a = min;
            true
        } else {
            false
        }
    }};
}

/// Update the value of the first argument to the maximum of the two arguments.
/// Returns true if the value is updated.
///
/// # Examples
///
/// ```
/// use algolib_rust::chmax;
///
/// let mut a = 2;
/// let b = 1;
/// assert!(!chmax!(a, b));
/// assert_eq!(a, 2);
///
/// let mut a = 1;
/// let b = 3;
/// assert!(chmax!(a, b));
/// assert_eq!(a, 3);
///
/// let mut v = vec![3, 1];
/// assert!(!chmax!(v[0], v[1]));
/// assert_eq!(v[0], 3);
///
/// let mut v = vec![3, 1];
/// assert!(chmax!(v[1], v[0]));
/// assert_eq!(v[1], 3);
/// ```
#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {{
        let b_val = $b.clone();
        let max = std::cmp::max($a, b_val);
        if $a < max {
            $a = max;
            true
        } else {
            false
        }
    }};
}
