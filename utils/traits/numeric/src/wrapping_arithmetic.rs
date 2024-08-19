pub trait WrappingAdd {
    fn wrapping_add(&self, rhs: &Self) -> Self
    where
        Self: Sized;
}

macro_rules! impl_wrapping_add {
    ($($t:ty)*) => {
        $(
            impl WrappingAdd for $t {
                fn wrapping_add(&self, rhs: &Self) -> Self {
                    <$t>::wrapping_add(*self, *rhs)
                }
            }
        )*
    }
}

impl_wrapping_add!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
