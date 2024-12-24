mod p521;

use curves::Config;
use elliptic_curve::{Numeric, Point};
use modular::Widened;
use numeric::Widen;

pub struct Ecdh<T> {
    config: Config<T>,
}

pub struct IntermediateValues<T> {
    public_key_1: T,
    public_key_2: T,
    shared_point: Point<T>,
}

impl<T: Numeric> Ecdh<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(config: Config<T>) -> Self {
        Self { config }
    }

    pub fn compute_shared_secret(&self, private_key_1: &T, private_key_2: &T) -> Option<T> {
        let g = &self.config.g;
        let curve = self.config.get_curve();
        let public_key_1 = curve.mul(g, private_key_1)?;
        let public_key_2 = curve.mul(g, private_key_2)?;

        let shared_point = curve.mul(&public_key_2, private_key_1)?;

        Some(shared_point.x)
    }
}

#[cfg(test)]
mod tests {
    use big_num::BigUint;
    use curves::P256;
    use numeric::FromStrRadix;

    use super::*;

    pub struct TestVector<'a> {
        pub private_key1: &'a str,
        pub private_key2: &'a str,
        pub shared_secret: &'a str,
    }

    #[macro_export]
    macro_rules! test_ecdh_vector {
        ($vector:expr, $ecdh: expr) => {{
            let private_key_1 = big_num::BigUint::from_be_hex($vector.private_key1).unwrap();
            let private_key_2 = big_num::BigUint::from_be_hex($vector.private_key2).unwrap();
            let shared_secret = big_num::BigUint::from_be_hex($vector.shared_secret).unwrap();

            let computed_secret = $ecdh
                .compute_shared_secret(&private_key_1, &private_key_2)
                .unwrap();

            if computed_secret != shared_secret {
                eprintln!(
                    "Test failed!\nExpected shared secret: {:x}\nActual shared secret: {:x}",
                    shared_secret, computed_secret
                );
            }

            assert_eq!(computed_secret, shared_secret);
        }};
    }
}
