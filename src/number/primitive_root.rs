//! Primitive root

use crate::utility::mod_ops::mod_pow;

/// Returns a primitive root of prime $`p`$.  
/// A primitive root of $`p`$ is an integer $`g`$ such that $`g^0, g^1, g^2, \dots, g^{p-2}`$ are all distinct modulo $`p`$.
///
/// # Constraints
///
/// - $`p`$ is prime
///
/// # Examples
///
/// ```
/// use algolib_rust::number::primitive_root::primitive_root;
/// assert_eq!(primitive_root(2), 1);
/// assert_eq!(primitive_root(3), 2);
/// assert_eq!(primitive_root(5), 2);
/// assert_eq!(primitive_root(7), 3);
/// assert_eq!(primitive_root(11), 2);
/// assert_eq!(primitive_root(13), 2);
/// assert_eq!(primitive_root(17), 3);
/// assert_eq!(primitive_root(19), 2);
/// assert_eq!(primitive_root(23), 5);
/// assert_eq!(primitive_root(29), 2);
/// ```
///
/// # Complexity
///
/// $`O(\sqrt{p})`$
pub fn primitive_root(p: u32) -> u32 {
    if p == 2 {
        1
    } else {
        const PN: usize = u32::BITS as usize;
        let mut pn: usize = 0;
        let mut qs = [0 as u32; PN];
        let mut q: u32 = 2;
        let mut pp: u32 = p - 1;
        while q * q <= pp {
            if pp % q == 0 {
                qs[pn] = q;
                pn += 1;
                while pp % q == 0 {
                    pp /= q;
                }
            }
            q += 1;
        }
        if pp != 1 {
            qs[pn] = q;
            pn += 1;
        }

        let mut g = 2;
        loop {
            let mut ok = true;
            let mut i = 0;
            while i < pn {
                if mod_pow(g, ((p - 1) / qs[i]) as u64, p) == 1 {
                    ok = false;
                    break;
                }
                i += 1
            }
            if ok {
                break;
            }
            g += 1;
        }
        g
    }
}
