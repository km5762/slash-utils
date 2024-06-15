pub trait CheckedMul {
    fn checked_mul(&self, rhs: &Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_mul {
    ($($t:ty)*) => {
        $(
            impl CheckedMul for $t {
                fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                    <$t>::checked_mul(*self, *rhs)
                }
            }
        )*
    }
}

impl_checked_mul!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
