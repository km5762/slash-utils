pub mod p256;

use alloc::{format, string::String};
use big_num::BigUInt;
use elliptic_curve::Point;
pub use p256::P256;

pub struct Config<T> {
    pub p: T,
    pub a: T,
    pub b: T,
    pub g: Point<T>,
    pub n: T,
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
    let big_num: BigUInt<N> = BigUInt::from_str_radix(s, 16);
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
            let curve = Curve::new($config.a, $config.b, $config.p);
            let k = BigUInt::from_str_radix($k, 10);
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
        pub k: &'a str,
        pub hash: &'a str,
        pub r: &'a str,
        pub s: &'a str,
    }

    #[macro_export]
    macro_rules! test_sign {
        ($test: expr) => {{
            let private_key = BigUInt::from_str_radix($test.private_key, 16);
            let k = BigUInt::from_str_radix($test.k, 16);
            let hash = BigUInt::from_str_radix($test.hash, 16);

            let ecdsa = crate::Ecdsa::new($test.config);
            let signature = ecdsa.sign(k, private_key, hash).unwrap();
            assert_eq!($test.r, signature.r.to_str_radix(16).to_uppercase());
            assert_eq!($test.s, signature.s.to_str_radix(16).to_uppercase());
        }};
    }
}
