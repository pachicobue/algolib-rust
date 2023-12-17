//! chmin,chmax関数を定義するモジュール

/// chmin関数を定義するトレイト
pub trait Chmin {
    /// `x`を`$\mathrm{min}(x,y)$`で更新できたらtrueを返す
    fn chmin(&mut self, y: Self) -> bool;
}

impl<T: PartialOrd> Chmin for T {
    fn chmin(&mut self, y: Self) -> bool {
        if *self > y {
            *self = y;
            true
        } else {
            false
        }
    }
}

/// chmax関数を定義するトレイト
pub trait Chmax {
    /// `x`を`$\mathrm{max}(x,y)$`で更新できたらtrueを返す
    fn chmax(&mut self, y: Self) -> bool;
}

impl<T: PartialOrd> Chmax for T {
    fn chmax(&mut self, y: Self) -> bool {
        if *self < y {
            *self = y;
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

        let mut a = 0.8;
        assert!(!a.chmin(10.0));
        assert_eq!(a, 0.8);
        assert!(a.chmin(-10.0));
        assert_eq!(a, -10.0);
    }

    #[test]
    fn test_chmax() {
        let mut a = 1;
        assert!(!a.chmax(-1));
        assert_eq!(a, 1);
        assert!(a.chmax(10));
        assert_eq!(a, 10);

        let mut a = 0.8;
        assert!(!a.chmax(-10.0));
        assert_eq!(a, 0.8);
        assert!(a.chmax(10.0));
        assert_eq!(a, 10.0);
    }
}
