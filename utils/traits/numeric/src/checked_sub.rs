pub trait CheckedSub {
    fn checked_sub(&self, rhs: &Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_sub {
    ($($t:ty)*) => {
        $(
            impl CheckedSub for $t {
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    <$t>::checked_sub(*self, *rhs)
                }
            }
        )*
    }
}

impl_checked_sub!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
