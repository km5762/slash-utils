use big_num::BigUInt;
use elliptic_curve::Point;

pub struct Config<T> {
    pub a: T,
    pub b: T,
    pub g: Point<T>,
    pub n: T,
}

fn p256() -> Config<BigUInt<8>> {
    Config {
        a: BigUInt::from_str_radix(
            "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
            16,
        ),
        b: BigUInt::from_str_radix(
            "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            16,
        ),
        g: Point::new(
            BigUInt::from_str_radix(
                "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
                16,
            ),
            BigUInt::from_str_radix(
                "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
                16,
            ),
        ),
        n: BigUInt::from_str_radix(
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
            16,
        ),
    }
}
