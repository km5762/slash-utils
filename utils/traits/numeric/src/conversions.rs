use core::num::ParseIntError;

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

pub trait FromStrRadix {
    type Error;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

macro_rules! impl_from_str_radix {
    ($($t:ty => $n:ty),*) => {
        $(
            impl FromStrRadix for $t {
                type Error = ParseIntError;
                fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::Error> {
                    <$t>::from_str_radix(src, radix)
                }
            }
        )*
    };
}

impl_from_str_radix! {
    i16 => i8,
    u16 => u8,
    i32 => i16,
    u32 => u16,
    i64 => i32,
    u64 => u32,
    i128 => i64,
    u128 => u64
}

pub trait FromBeBytes {
    type Bytes;

    fn from_be_bytes(bytes: &Self::Bytes) -> Self;
}

macro_rules! impl_from_be_bytes {
    ($($t:ty => $bytes:ty),*) => {
        $(
            impl FromBeBytes for $t {
                type Bytes = $bytes;
                fn from_be_bytes(bytes: &Self::Bytes) -> Self {
                    Self::from_be_bytes(*bytes)
                }
            }
        )*
    };
}

impl_from_be_bytes! {
    u16 => [u8; 2],
    u32 => [u8; 4],
    u64 => [u8; 8],
    u128 => [u8; 16],
    i16 => [u8; 2],
    i32 => [u8; 4],
    i64 => [u8; 8],
    i128 => [u8; 16]
}

pub trait ToBeBytes {
    type Bytes;

    fn to_be_bytes(&self) -> Self::Bytes;
}

macro_rules! impl_to_be_bytes {
    ($($t:ty => $bytes:ty),*) => {
        $(
            impl ToBeBytes for $t {
                type Bytes = $bytes;
                fn to_be_bytes(&self) -> Self::Bytes {
                    Self::to_be_bytes(*self)
                }
            }
        )*
    };
}

impl_to_be_bytes! {
    u16 => [u8; 2],
    u32 => [u8; 4],
    u64 => [u8; 8],
    u128 => [u8; 16],
    i16 => [u8; 2],
    i32 => [u8; 4],
    i64 => [u8; 8],
    i128 => [u8; 16]
}
