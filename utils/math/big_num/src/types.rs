use crate::BigUint;
use numeric::{FromBeBytes, Narrow, ToBeBytes, Widen};

pub const RADIX: u64 = u32::MAX as u64 + 1;

pub type U256 = BigUint<8>;
pub type U512 = BigUint<16>;
pub type U1024 = BigUint<32>;
pub type U2048 = BigUint<64>;
pub type U4096 = BigUint<128>;

macro_rules! impl_narrow {
    ($narrow_size: expr) => {
        impl Narrow for BigUint<{ $narrow_size * 2 }> {
            type Output = BigUint<$narrow_size>;

            fn narrow(&self) -> BigUint<$narrow_size> {
                let mut limbs = [0; $narrow_size];
                limbs.clone_from_slice(&self.limbs[..$narrow_size]);

                BigUint::new(limbs)
            }
        }
    };
}

impl_narrow!(8);
impl_narrow!(16);
impl_narrow!(32);
impl_narrow!(64);
impl_narrow!(128);

macro_rules! impl_widen {
    ($narrow_size: expr) => {
        impl Widen for BigUint<{ $narrow_size }> {
            type Output = BigUint<{ $narrow_size * 2 }>;

            fn widen(&self) -> BigUint<{ $narrow_size * 2 }> {
                let mut limbs = [0; $narrow_size * 2];
                limbs[..$narrow_size].clone_from_slice(&self.limbs);

                BigUint::new(limbs)
            }
        }
    };
}

impl_widen!(4);
impl_widen!(8);
impl_widen!(16);
impl_widen!(32);
impl_widen!(64);

macro_rules! impl_from_be_bytes {
    ($($t:ty)*) => {
        $(
            impl FromBeBytes for $t {
                type Bytes = [u8; <$t>::BYTES];

                fn from_be_bytes(bytes: &Self::Bytes) -> Self {
                    let mut limbs = [0u32; <$t>::LIMBS];

                    for i in (0..<$t>::LIMBS).rev() {
                        limbs[i] = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap());
                    }

                    Self::new(limbs)
                }
            }
        )*
    }
}

impl_from_be_bytes!(U256 U512 U1024 U2048 U4096);

macro_rules! impl_to_be_bytes {
    ($($t:ty)*) => {
        $(
            impl ToBeBytes for $t {
                type Bytes = [u8; <$t>::BYTES];

                fn to_be_bytes(&self) -> Self::Bytes {
                    let mut bytes = [0u8; <$t>::BYTES];

                    for i in 0..<$t>::LIMBS {
                        bytes[4*i..][..4].copy_from_slice(&self.limbs[i].to_le_bytes());
                    }

                    bytes
                }
            }
        )*
    }
}

impl_to_be_bytes!(U256 U512 U1024 U2048 U4096);
