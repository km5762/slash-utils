#![no_std]

mod p256;
mod p384;
mod p521;
pub mod wasm_adapter;

extern crate alloc;

use curves::Config;
use elliptic_curve::{Curve, Numeric, Point};
use modular::{Ring, Widened};
use numeric::Widen;

pub struct Ecdsa<T> {
    config: Config<T>,
}

#[derive(Debug)]
pub enum SigningError {
    InvalidPoint,
    NoInvK,
    ZeroingK,
}

pub struct SigningIntermediateValues<T> {
    generated_point: Point<T>,
    signature: (T, T),
}

pub struct VerifyingIntermediateValues<T> {
    u: Option<(T, T)>,
    generated_point: Option<Point<T>>,
    valid: bool,
}

impl<T> Default for VerifyingIntermediateValues<T> {
    fn default() -> Self {
        Self {
            u: None,
            generated_point: None,
            valid: false,
        }
    }
}

impl<T: Numeric> Ecdsa<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub const fn new(config: Config<T>) -> Self {
        Self { config }
    }

    pub fn sign(
        &self,
        k: &T,
        key: &T,
        hash: &T,
    ) -> Result<SigningIntermediateValues<T>, SigningError> {
        let Config { g, n, .. } = &self.config;

        let curve = self.config.get_curve();
        let ring = self.config.get_ring();

        let point = match curve.mul(&g, k) {
            Some(point) => point,
            None => return Err(SigningError::InvalidPoint),
        };

        let r = point.x.rem_euclid(n);

        if r == T::zero() {
            return Err(SigningError::ZeroingK);
        }

        let k_inv = match ring.inv(*k) {
            Some(inv) => inv,
            None => return Err(SigningError::NoInvK),
        };

        let s = ring.mul(k_inv, ring.add(*hash, ring.mul(r, *key)));

        Ok(SigningIntermediateValues {
            generated_point: point,
            signature: (r, s),
        })
    }

    pub fn verify(
        &self,
        key: &Point<T>,
        hash: &T,
        signature: &(T, T),
    ) -> VerifyingIntermediateValues<T> {
        let Config { p, a, b, g, n, .. } = &self.config;
        let mut intermediate_values = VerifyingIntermediateValues::default();

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

        let (r, s) = *signature;

        let s_inv = match ring.inv(s) {
            Some(inv) => inv,
            None => return intermediate_values,
        };

        let u1 = ring.mul(*hash, s_inv);
        let u2 = ring.mul(r, s_inv);
        intermediate_values.u = Some((u1, u2));

        let point1 = match curve.mul(g, &u1) {
            Some(point) => point,
            None => return intermediate_values,
        };

        let point2 = match curve.mul(key, &u2) {
            Some(point) => point,
            None => return intermediate_values,
        };

        let random_point = match curve.add(&point1, &point2) {
            Some(point) => point,
            None => return intermediate_values,
        };

        intermediate_values.generated_point = Some(random_point.clone());

        if r == random_point.x {
            intermediate_values.valid = true;
            intermediate_values
        } else {
            intermediate_values
        }
    }
}

#[cfg(test)]
mod test {
    use curves::Config;

    #[macro_export]
    macro_rules! test_point_generation {
        ($config:expr, $k:expr, $expected_x:expr, $expected_y:expr) => {{
            let curve = elliptic_curve::Curve::new($config.a, $config.b, $config.p);
            let k = numeric::FromStrRadix::from_str_radix($k, 10).unwrap();
            let result = curve.mul(&$config.g, &k).unwrap();
            let x = result.x.to_str_radix(16);
            let y = result.y.to_str_radix(16);

            assert_eq!($expected_x.to_lowercase(), x.to_lowercase());
            assert_eq!($expected_y.to_lowercase(), y.to_lowercase());
        }};
    }

    pub struct SignTest<'a, T> {
        pub config: Config<T>,
        pub private_key: &'a str,
        pub public_key: (&'a str, &'a str),
        pub k: &'a str,
        pub hash: &'a str,
        pub signature: (&'a str, &'a str),
    }

    #[macro_export]
    macro_rules! test_sign {
        ($test: expr) => {{
            let ecdsa = crate::Ecdsa::new($test.config);

            let k = numeric::FromStrRadix::from_str_radix($test.k, 16).unwrap();
            let private_key = numeric::FromStrRadix::from_str_radix($test.private_key, 16).unwrap();
            let hash = numeric::FromStrRadix::from_str_radix($test.hash, 16).unwrap();

            let signature = ecdsa.sign(&k, &private_key, &hash).unwrap().signature;
            assert_eq!(
                $test.signature.0.to_uppercase(),
                signature.0.to_str_radix(16).to_uppercase()
            );
            assert_eq!(
                $test.signature.1.to_uppercase(),
                signature.1.to_str_radix(16).to_uppercase()
            );

            let public_key = elliptic_curve::Point::new(
                numeric::FromStrRadix::from_str_radix($test.public_key.0, 16).unwrap(),
                numeric::FromStrRadix::from_str_radix($test.public_key.1, 16).unwrap(),
            );

            assert!(ecdsa.verify(&public_key, &hash, &signature).valid);
        }};
    }
}
