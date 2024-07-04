use alloc::{format, string::String};
use big_num::BigUInt;
use elliptic_curve::Point;
use modular::Ring;

pub struct Config<T> {
    pub a: T,
    pub b: T,
    pub g: Point<T>,
    pub n: T,
}

pub const P256: Config<BigUInt<8>> = Config {
    a: BigUInt::new([4294967292, 4294967295, 4294967295, 0, 0, 0, 1, 4294967295]),
    b: BigUInt::new([
        668098635, 1003371582, 3428036854, 1696401072, 1989707452, 3018571093, 2855965671,
        1522939352,
    ]),
    g: Point::new(
        BigUInt::new([
            3633889942, 4104206661, 770388896, 1996717441, 1671708914, 4173129445, 3777774151,
            1796723186,
        ]),
        BigUInt::new([
            935285237, 3417718888, 1798397646, 734933847, 2081398294, 2397563722, 4263149467,
            1340293858,
        ]),
    ),
    n: BigUInt::new([4294967295, 4294967295, 4294967295, 0, 0, 0, 1, 4294967295]),
};

fn generate_config<const N: usize>(a: &str, b: &str, gx: &str, gy: &str, n: &str) -> String {
    format!(
        "Config {{ a: {}, b: {}, g: Point::new({}, {}), n: {}}};",
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
fn test() {
    assert_eq!(
        "",
        generate_config::<8>(
            "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
            "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
            "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
        )
    );
}
