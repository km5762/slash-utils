pub trait LeadingZeros {
    fn leading_zeros(&self) -> u32;
}

macro_rules! impl_leading_zeros {
    ($($t:ty)*) => {
        $(
            impl LeadingZeros for $t {
                fn leading_zeros(&self) -> u32 {
                    <$t>::leading_zeros(*self)
                }
            }
        )*
    }
}

impl_leading_zeros!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);

pub trait Bit {
    fn bit(&self, index: usize) -> bool;
}

macro_rules! impl_bit {
    ($($t:ty)*) => {
        $(
            impl Bit for $t {
                fn bit(&self, index: usize) -> bool {
                    ((*self >> index) & 1) != 0
                }
            }
        )*
    }
}

impl_bit!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);

pub trait SetBit {
    fn set_bit(&mut self, index: usize, value: bool);
}

macro_rules! impl_set_bit {
    ($($t:ty)*) => {
        $(
            impl SetBit for $t {
                fn set_bit(&mut self, index: usize, value: bool) {
                    let mask = 1 << index;
                    *self = (*self & !mask) | ((value as $t) * mask);
                }
            }
        )*
    }
}

impl_set_bit!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);
