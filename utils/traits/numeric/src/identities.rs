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

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
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

impl_zero!(usize, 0);
impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);

impl_zero!(isize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
