pub trait CheckedAdd {
    fn checked_add(&self, rhs: &Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_add {
    ($($t:ty)*) => {
        $(
            impl CheckedAdd for $t {
                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    <$t>::checked_add(*self, *rhs)
                }
            }
        )*
    }
}

impl_checked_add!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
