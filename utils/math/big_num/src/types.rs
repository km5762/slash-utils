use crate::BigUInt;
use numeric::{Narrow, Widen};

pub const RADIX: u64 = u32::MAX as u64 + 1;

pub type U256 = BigUInt<8>;
pub type U512 = BigUInt<16>;
pub type U1024 = BigUInt<32>;
pub type U2048 = BigUInt<64>;
pub type U4096 = BigUInt<128>;

macro_rules! impl_narrow {
    ($narrow_size: expr) => {
        impl Narrow for BigUInt<{ $narrow_size * 2 }> {
            type Output = BigUInt<$narrow_size>;

            fn narrow(&self) -> BigUInt<$narrow_size> {
                let mut limbs = [0; $narrow_size];
                limbs.clone_from_slice(&self.limbs[..$narrow_size]);

                BigUInt::new(limbs)
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
        impl Widen for BigUInt<{ $narrow_size }> {
            type Output = BigUInt<{ $narrow_size * 2 }>;

            fn widen(&self) -> BigUInt<{ $narrow_size * 2 }> {
                let mut limbs = [0; $narrow_size * 2];
                limbs[..$narrow_size].clone_from_slice(&self.limbs);

                BigUInt::new(limbs)
            }
        }
    };
}

impl_widen!(4);
impl_widen!(8);
impl_widen!(16);
impl_widen!(32);
impl_widen!(64);
