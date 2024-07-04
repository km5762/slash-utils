#![no_std]

pub mod configs;

use core::fmt;
extern crate alloc;

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
    use alloc::format;
    use big_num::BigUInt;
    use configs::P256;

    use super::*;

    macro_rules! test_point_generation {
        ($config:expr, $k:expr, $expected_x:expr, $expected_y:expr) => {{
            let curve = Curve::new($config.a, $config.b, $config.n);
            let k = BigUInt::from_str_radix($k, 16);
            let result = curve.mul(&$config.g, k).unwrap();
            let x = result.x.to_str_radix(16);
            let y = result.y.to_str_radix(16);

            assert_eq!($expected_x.to_lowercase(), x.to_lowercase());
            assert_eq!($expected_y.to_lowercase(), y.to_lowercase());
        }};
    }

    #[test]
    fn p256_point_generation() {
        test_point_generation!(
            P256,
            "2",
            "7CF27B188D034F7E8A52380304B51AC3C08969E277F21B35A60B48FC47669978",
            "7775510DB8ED040293D9AC69F7430DBBA7DADE63CE982299E04B79D227873D1"
        );
        test_point_generation!(
            P256,
            "3",
            "5ECBE4D1A6330A44C8F7EF951D4BF165E6C6B721EFADA985FB41661BC6E7FD6C",
            "8734640C4998FF7E374B06CE1A64A2ECD82AB036384FB83D9A79B127A27D5032"
        );
        test_point_generation!(
            P256,
            "4",
            "E2534A3532D08FBBA02DDE659EE62BD0031FE2DB785596EF509302446B030852",
            "E0F1575A4C633CC719DFEE5FDA862D764EFC96C3F30EE0055C42C23F184ED8C6"
        );
        test_point_generation!(
            P256,
            "5",
            "51590B7A515140D2D784C85608668FDFEF8C82FD1F5BE52421554A0DC3D033ED",
            "E0C17DA8904A727D8AE1BF36BF8A79260D012F00D4D80888D1D0BB44FDA16DA4"
        );
        test_point_generation!(
            P256,
            "6",
            "B01A172A76A4602C92D3242CB897DDE3024C740DEBB215B4C6B0AAE93C2291A9",
            "E85C10743237DAD56FEC0E2DFBA703791C00F7701C7E16BDFD7C48538FC77FE2"
        );
        test_point_generation!(
            P256,
            "7",
            "8E533B6FA0BF7B4625BB30667C01FB607EF9F8B8A80FEF5B300628703187B2A3",
            "73EB1DBDE03318366D069F83A6F5900053C73633CB041B21C55E1A86C1F400B4"
        );
        test_point_generation!(
            P256,
            "8",
            "62D9779DBEE9B0534042742D3AB54CADC1D238980FCE97DBB4DD9DC1DB6FB393",
            "AD5ACCBD91E9D8244FF15D771167CEE0A2ED51F6BBE76A78DA540A6A0F09957E"
        );
        test_point_generation!(
            P256,
            "9",
            "EA68D7B6FEDF0B71878938D51D71F8729E0ACB8C2C6DF8B3D79E8A4B90949EE0",
            "2A2744C972C9FCE787014A964A8EA0C84D714FEAA4DE823FE85A224A4DD048FA"
        );
        test_point_generation!(
            P256,
            &format!("{:X}", 10),
            "CEF66D6B2A3A993E591214D1EA223FB545CA6C471C48306E4C36069404C5723F",
            "878662A229AAAE906E123CDD9D3B4C10590DED29FE751EEECA34BBAA44AF0773"
        );
        test_point_generation!(
            P256,
            &format!("{:X}", 11),
            "3ED113B7883B4C590638379DB0C21CDA16742ED0255048BF433391D374BC21D1",
            "9099209ACCC4C8A224C843AFA4F4C68A090D04DA5E9889DAE2F8EEFCE82A3740"
        );
        test_point_generation!(
            P256,
            &format!("{:X}", 12),
            "741DD5BDA817D95E4626537320E5D55179983028B2F82C99D500C5EE8624E3C4",
            "770B46A9C385FDC567383554887B1548EEB912C35BA5CA71995FF22CD4481D3"
        );
        test_point_generation!(
            P256,
            &format!("{:X}", 13),
            "177C837AE0AC495A61805DF2D85EE2FC792E284B65EAD58A98E15D9D46072C01",
            "63BB58CD4EBEA558A24091ADB40F4E7226EE14C3A1FB4DF39C43BBE2EFC7BFD8"
        );
        test_point_generation!(
            P256,
            &format!("{:X}", 14),
            "54E77A001C3862B97A76647F4336DF3CF126ACBE7A069C5E5709277324D2920B",
            "F599F1BB29F4317542121F8C05A2E7C37171EA77735090081BA7C82F60D0B375"
        );
        test_point_generation!(
            P256,
            "18EBBB95EED0E13",
            "339150844EC15234807FE862A86BE77977DBFB3AE3D96F4C22795513AEAAB82F",
            "B1C14DDFDC8EC1B2583F51E85A5EB3A155840F2034730E9B5ADA38B674336A21"
        );
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
