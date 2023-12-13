//! chmin,chmax関数を定義するモジュール

/// chmin関数を定義するトレイト
pub trait Chmin {
    fn chmin(&mut self, other: Self) -> bool;
}

impl<T: PartialOrd> Chmin for T {
    fn chmin(&mut self, other: Self) -> bool {
        if *self > other {
            *self = other;
            true
        } else {
            false
        }
    }
}

/// chmax関数を定義するトレイト
pub trait Chmax {
    fn chmax(&mut self, other: Self) -> bool;
}

impl<T: PartialOrd> Chmax for T {
    fn chmax(&mut self, other: Self) -> bool {
        if *self < other {
            *self = other;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chmin() {
        let mut a = 1;
        assert!(!a.chmin(10));
        assert_eq!(a, 1);
        assert!(a.chmin(-1));
        assert_eq!(a, -1);
    }

    #[test]
    fn test_chmax() {
        let mut a = 1;
        assert!(!a.chmax(-1));
        assert_eq!(a, 1);
        assert!(a.chmax(10));
        assert_eq!(a, 10);
    }
}
