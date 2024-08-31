pub mod p256;
pub mod p384;
pub mod p521;

use elliptic_curve::{Curve, Numeric, Point};
use modular::Widened;
use numeric::Widen;
pub use p256::P256;
pub use p384::P384;

pub struct Config<T> {
    pub p: T,
    pub a: T,
    pub b: T,
    pub g: Point<T>,
    pub n: T,
    _private: (),
}

pub struct InvalidGeneratorError;

impl<T: Numeric> Config<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(p: T, a: T, b: T, g: Point<T>, n: T) -> Result<Self, InvalidGeneratorError> {
        let curve = Curve::new(a, b, p);

        if !curve.is_valid_point(&g) {
            return Err(InvalidGeneratorError);
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

#[cfg(test)]
mod tests {
    use alloc::{format, string::String};
    use big_num::BigUint;
    use numeric::FromStrRadix;

    use super::Config;

    fn generate_config<const N: usize>(
        p: &str,
        a: &str,
        b: &str,
        g: (&str, &str),
        n: &str,
    ) -> String {
        format!(
            "Config {{ p: {}, a: {}, b: {}, g: Point::new({}, {}), n: {}, _private: ()}};",
            generate_big_num_constructor::<N>(p),
            generate_big_num_constructor::<N>(a),
            generate_big_num_constructor::<N>(b),
            generate_big_num_constructor::<N>(g.0),
            generate_big_num_constructor::<N>(g.1),
            generate_big_num_constructor::<N>(n)
        )
    }

    fn generate_big_num_constructor<const N: usize>(s: &str) -> String {
        let big_num: BigUint<N> = BigUint::from_str_radix(s, 16).unwrap();
        format!("BigUint::new({:?})", big_num.to_limbs())
    }

    #[test]
    fn gen_p256() {
        let cfg = generate_config::<20>(
        "1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc",
        "51953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00",
        ("c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66", "11839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650"),
        "1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409",
    );

        assert_eq!(cfg, "")
    }

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

            let signature = ecdsa.sign(&k, &private_key, &hash).unwrap().signature;
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

            assert!(ecdsa.verify(&public_key, &hash, &signature).valid);
        }};
    }
}
