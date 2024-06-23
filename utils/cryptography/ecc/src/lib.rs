#![no_std]

pub mod configs;

use core::fmt::{self};

use big_num::BigUInt;
use configs::Config;
use elliptic_curve::{Curve, Numeric, Point};
use modular::{Ring, Widened};
use numeric::Widen;

#[derive(PartialEq, Debug)]
struct Signature<T> {
    r: T,
    s: T,
}

impl<T> Signature<T> {
    fn new(r: T, s: T) -> Self {
        Signature { r, s }
    }
}

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidGenerator,
    InvalidK,
    NoInverseK,
    InvalidPoint,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidGenerator => write!(f, "the generator point is not on the curve"),
            Error::InvalidK => write!(f, "the random k value selected results in a zero point"),
            Error::NoInverseK => write!(
                f,
                "the random k value selected has no multiplicitive inverse with the given modulus"
            ),
            Error::InvalidPoint => write!(
                f,
                "an invalid point was encountered while generating the signature"
            ),
        }
    }
}

fn sign<T: Numeric>(config: Config<T>, k: T, private_key: T, hash: T) -> Result<Signature<T>, Error>
where
    <T as Widen>::Output: Widened<T>,
{
    let Config { a, b, g, n } = config;

    let curve = Curve::new(a, b, n);
    let ring = Ring::new(n);

    if !curve.is_valid_point(&g) {
        return Err(Error::InvalidGenerator);
    }

    let point = match curve.mul(&g, k) {
        Some(p) => p,
        None => return Err(Error::InvalidPoint),
    };

    let r = point.x;

    if r == T::zero() {
        return Err(Error::InvalidK);
    }

    let k_inv = match ring.inv(k) {
        Some(inv) => inv,
        None => return Err(Error::NoInverseK),
    };

    let s = ring.mul(k_inv, ring.add(hash, ring.mul(r, private_key)));

    Ok(Signature { r, s })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p256() {
        let a: BigUInt<16> = BigUInt::from_str_radix(
            "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
            16,
        );
        let b: BigUInt<16> = BigUInt::from_str_radix(
            "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            16,
        );
        let modulus: BigUInt<16> = BigUInt::from_str_radix(
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
            16,
        );
        let gx: BigUInt<16> = BigUInt::from_str_radix(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
            16,
        );
        let gy: BigUInt<16> = BigUInt::from_str_radix(
            "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
            16,
        );
        let x: BigUInt<16> = BigUInt::from_str_radix(
            "7CF27B188D034F7E8A52380304B51AC3C08969E277F21B35A60B48FC47669978",
            16,
        );
        let y: BigUInt<16> = BigUInt::from_str_radix(
            "07775510DB8ED040293D9AC69F7430DBBA7DADE63CE982299E04B79D227873D1",
            16,
        );
        let k: BigUInt<16> = BigUInt::from(2);
        let curve = Curve::new(a, b, modulus);
        let g = Point::new(gx, gy);
        let result = curve.mul(&g, k);
        assert_eq!(Some(Point::new(x, y)), result);
    }

    // #[test]
    // fn sign_256() {
    //     let a: BigUInt<8> = BigUInt::from_str_radix(
    //         "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
    //         16,
    //     );
    //     let b: BigUInt<8> = BigUInt::from_str_radix(
    //         "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
    //         16,
    //     );
    //     let modulus: BigUInt<8> = BigUInt::from_str_radix(
    //         "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
    //         16,
    //     );
    //     let gx: BigUInt<8> = BigUInt::from_str_radix(
    //         "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
    //         16,
    //     );
    //     let gy: BigUInt<8> = BigUInt::from_str_radix(
    //         "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
    //         16,
    //     );
    //     let gy_string = gy.to_str_radix(16);
    //     let g = Point::new(gx, gy);
    //     let private_key: BigUInt<8> = BigUInt::from_str_radix(
    //         "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721",
    //         16,
    //     );
    //     let k: BigUInt<8> = BigUInt::from_str_radix(
    //         "882905F1227FD620FBF2ABF21244F0BA83D0DC3A9103DBBEE43A1FB858109DB4",
    //         16,
    //     );
    //     let hash: BigUInt<8> =
    //         BigUInt::from_str_radix("8151325dcdbae9e0ff95f9f9658432dbedfdb209", 16);

    //     let signature = sign(a, b, modulus, g, k, private_key, hash);
    //     assert_eq!(
    //         "61340C88C3AAEBEB4F6D667F672CA9759A6CCAA9FA8811313039EE4A35471D32",
    //         signature.unwrap().r.to_str_radix(16)
    //     );
    // }
}
