pub trait Widen {
    type Output;
    fn widen(&self) -> Self::Output;
}

macro_rules! impl_widen {
    ($($t:ty => $w:ty),*) => {
        $(
            impl Widen for $t {
                type Output = $w;
                fn widen(&self) -> Self::Output {
                    *self as $w
                }
            }
        )*
    };
}

impl_widen! {
    i8 => i16,
    u8 => u16,
    i16 => i32,
    u16 => u32,
    i32 => i64,
    u32 => u64,
    i64 => i128,
    u64 => u128
}

pub trait Narrow {
    type Output;
    fn narrow(&self) -> Self::Output;
}

macro_rules! impl_narrow {
    ($($t:ty => $n:ty),*) => {
        $(
            impl Narrow for $t {
                type Output = $n;
                fn narrow(&self) -> Self::Output {
                    *self as $n
                }
            }
        )*
    };
}

impl_narrow! {
    i16 => i8,
    u16 => u8,
    i32 => i16,
    u32 => u16,
    i64 => i32,
    u64 => u32,
    i128 => i64,
    u128 => u64
}
