/// 有界な数値型のトレイト
pub trait Bounded: PartialOrd {
    /// 最小値を返す
    fn min_value() -> Self;
    /// 最大値を返す
    fn max_value() -> Self;
}
macro_rules! impl_bounded_prim {
    ($t:ty) => {
        impl Bounded for $t {
            fn min_value() -> Self {
                <$t>::MIN
            }
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}
impl_bounded_prim!(isize);
impl_bounded_prim!(i8);
impl_bounded_prim!(i16);
impl_bounded_prim!(i32);
impl_bounded_prim!(i64);
#[cfg(has_i128)]
impl_bounded_prim!(i128);
impl_bounded_prim!(usize);
impl_bounded_prim!(u8);
impl_bounded_prim!(u16);
impl_bounded_prim!(u32);
impl_bounded_prim!(u64);
#[cfg(has_i128)]
impl_bounded_prim!(u128);

impl_bounded_prim!(f32);
impl_bounded_prim!(f64);
