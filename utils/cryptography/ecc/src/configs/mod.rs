pub mod p256;

use alloc::{format, string::String};
use big_num::BigUint;
use elliptic_curve::{Curve, Numeric, Point};
use modular::{Ring, Widened};
use numeric::{FromStrRadix, Widen};
pub use p256::P256;

pub struct Config<T> {
    pub p: T,
    pub a: T,
    pub b: T,
    pub g: Point<T>,
    pub n: T,
    _private: (),
}

struct InvalidGenerator;

impl<T: Numeric> Config<T>
where
    <T as Widen>::Output: Widened<T>,
{
    fn new(p: T, a: T, b: T, g: Point<T>, n: T) -> Result<Self, InvalidGenerator> {
        let curve = Curve::new(a, b, p);

        if !curve.is_valid_point(&g) {
            return Err(InvalidGenerator);
        }

        Ok(Self {
            p,
            a,
            b,
            g,
            n,
            _private: (),
        })
    }
}

fn generate_config<const N: usize>(
    p: &str,
    a: &str,
    b: &str,
    gx: &str,
    gy: &str,
    n: &str,
) -> String {
    format!(
        "Config {{ p: {}, a: {}, b: {}, g: Point::new({}, {}), n: {}}};",
        generate_big_num_constructor::<N>(p),
        generate_big_num_constructor::<N>(a),
        generate_big_num_constructor::<N>(b),
        generate_big_num_constructor::<N>(gx),
        generate_big_num_constructor::<N>(gy),
        generate_big_num_constructor::<N>(n)
    )
}

fn generate_big_num_constructor<const N: usize>(s: &str) -> String {
    let big_num: BigUint<N> = BigUint::from_str_radix(s, 16).unwrap();
    format!("BigUInt::new({:?})", big_num.to_limbs())
}

#[test]
fn gen_p256() {
    let cfg = generate_config::<8>(
        "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
        "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
        "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
        "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
        "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
        "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
    );

    assert_eq!(cfg, "")
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[macro_export]
    macro_rules! test_point_generation {
        ($config:expr, $k:expr, $expected_x:expr, $expected_y:expr) => {{
            let curve = elliptic_curve::Curve::new($config.a, $config.b, $config.p);
            let k = numeric::FromStrRadix::from_str_radix($k, 10).unwrap();
            let result = curve.mul(&$config.g, k).unwrap();
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

            let signature = ecdsa.sign(&k, &private_key, &hash).unwrap();
            assert_eq!(
                $test.signature.0.to_uppercase(),
                signature.0.to_str_radix(16).to_uppercase()
            );
            assert_eq!(
                $test.signature.1.to_uppercase(),
                signature.1.to_str_radix(16).to_uppercase()
            );

            let public_key = Point::new(
                numeric::FromStrRadix::from_str_radix($test.public_key.0, 16).unwrap(),
                numeric::FromStrRadix::from_str_radix($test.public_key.1, 16).unwrap(),
            );

            assert!(ecdsa.verify(&public_key, &hash, &signature));
        }};
    }
}
