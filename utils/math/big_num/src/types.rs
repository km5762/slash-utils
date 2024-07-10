use crate::BigUint;
use numeric::{FromBeBytes, Narrow, ToBeBytes, Widen};

pub const RADIX: u64 = u32::MAX as u64 + 1;

pub type U192 = BigUint<6>;
pub type U256 = BigUint<8>;
pub type U320 = BigUint<10>;
pub type U384 = BigUint<12>;
pub type U512 = BigUint<16>;
pub type U640 = BigUint<20>;
pub type U768 = BigUint<24>;
pub type U1024 = BigUint<32>;
pub type U1280 = BigUint<40>;
pub type U2048 = BigUint<64>;
pub type U4096 = BigUint<128>;

macro_rules! impl_narrow {
    ($($t:ty)*) => {
        $(
            impl Narrow for $t {
                type Output = BigUint<{<$t>::LIMBS / 2}>;

                fn narrow(&self) -> Self::Output {
                    let mut limbs = [0; <$t>::LIMBS / 2];
                    limbs.clone_from_slice(&self.limbs[..(<$t>::LIMBS / 2)]);

                    BigUint::new(limbs)
                }
            }
        )*
    };
}

impl_narrow!(U192 U256 U320 U384 U512 U640 U768 U1024 U1280 U2048 U4096);

macro_rules! impl_widen {
    ($($t:ty)*) => {
        $(
            impl Widen for $t {
                type Output = BigUint<{ <$t>::LIMBS * 2 }>;

                fn widen(&self) -> Self::Output {
                    let mut limbs = [0; <$t>::LIMBS * 2];
                    limbs[..(<$t>::LIMBS)].clone_from_slice(&self.limbs);

                    BigUint::new(limbs)
                }
            }
        )*
    };
}

impl_widen!(U192 U256 U320 U384 U512 U640 U768 U1024 U1280 U2048 U4096);

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

impl_from_be_bytes!(U192 U256 U320 U384 U512 U640 U768 U1024 U1280 U2048 U4096);

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

impl_to_be_bytes!(U192 U256 U320 U384 U512 U640 U768 U1024 U1280 U2048 U4096);
