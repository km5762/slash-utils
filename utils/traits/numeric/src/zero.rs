pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_one {
    ($($t:ty, $v:expr)*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    $v
                }
            }
        )*
    }
}

impl_one!(usize, 0);
impl_one!(u8, 0);
impl_one!(u16, 0);
impl_one!(u32, 0);
impl_one!(u64, 0);
impl_one!(u128, 0);

impl_one!(isize, 0);
impl_one!(i8, 0);
impl_one!(i16, 0);
impl_one!(i32, 0);
impl_one!(i64, 0);
impl_one!(i128, 0);
