pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($($t:ty, $v:expr)*) => {
        $(
            impl One for $t {
                fn one() -> Self {
                    $v
                }
            }
        )*
    }
}

impl_one!(usize, 1);
impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);

impl_one!(isize, 1);
impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);
