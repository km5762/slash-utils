pub trait LeadingZeros {
    fn leading_zeros(self) -> u32;
}

macro_rules! impl_leading_zeros {
    ($($t:ty)*) => {
        $(
            impl LeadingZeros for $t {
                fn leading_zeros(self) -> u32 {
                    <$t>::leading_zeros(self)
                }
            }
        )*
    }
}

impl_leading_zeros!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
