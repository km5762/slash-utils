use crate::BigUInt;
use numeric::WideningMul;

pub const RADIX: u64 = u32::MAX as u64 + 1;

pub type U256 = BigUInt<8>;
pub type U512 = BigUInt<16>;
pub type U1024 = BigUInt<32>;
pub type U2048 = BigUInt<64>;
pub type U4096 = BigUInt<128>;

macro_rules! impl_widening_mul {
    ($Type:ident, $Size:expr) => {
        impl WideningMul for $Type<$Size> {
            type Output = $Type<{ $Size * 2 }>;

            fn widening_mul(&self, rhs: &Self) -> Self::Output {
                let mut w = [0; $Size * 2];
                let mut k = 0;

                for j in 0..$Size {
                    k = 0;
                    if rhs.limbs[j] == 0 {
                        continue;
                    }
                    for i in 0..$Size {
                        let t = u64::from(self.limbs[i]) * u64::from(rhs.limbs[j])
                            + u64::from(w[i + j])
                            + k;
                        w[i + j] = (t % RADIX) as u32;
                        k = t / RADIX;
                    }
                    w[$Size + j] = k as u32;
                }

                $Type::new(w)
            }
        }
    };
}

impl_widening_mul!(BigUInt, 8);
impl_widening_mul!(BigUInt, 16);
impl_widening_mul!(BigUInt, 32);
impl_widening_mul!(BigUInt, 64);
impl_widening_mul!(BigUInt, 128);
