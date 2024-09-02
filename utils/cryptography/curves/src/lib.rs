#![no_std]

use big_num::{types::{U384, U640}, BigUint};
use elliptic_curve::{Curve, Numeric, Point};
use modular::Widened;
use numeric::Widen;

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

pub const P256: Config<BigUint<8>> = Config {
    p: BigUint::new([4294967295, 4294967295, 4294967295, 0, 0, 0, 1, 4294967295]),
    a: BigUint::new([4294967292, 4294967295, 4294967295, 0, 0, 0, 1, 4294967295]),
    b: BigUint::new([
        668098635, 1003371582, 3428036854, 1696401072, 1989707452, 3018571093, 2855965671,
        1522939352,
    ]),
    g: Point::new(
        BigUint::new([
            3633889942, 4104206661, 770388896, 1996717441, 1671708914, 4173129445, 3777774151,
            1796723186,
        ]),
        BigUint::new([
            935285237, 3417718888, 1798397646, 734933847, 2081398294, 2397563722, 4263149467,
            1340293858,
        ]),
    ),
    n: BigUint::new([
        4234356049, 4089039554, 2803342980, 3169254061, 4294967295, 4294967295, 0, 4294967295,
    ]),
    _private: (),
};

pub const P384: Config<U384> = Config {
    p: BigUint::new([
        4294967295, 0, 0, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295,
    ]),
    a: BigUint::new([
        4294967292, 0, 0, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295,
    ]),
    b: BigUint::new([
        3555470063, 713410797, 2318324125, 3327539597, 1343457114, 51644559, 4269883666, 404593774,
        3824692505, 2559444331, 3795773412, 3006345127,
    ]),
    g: Point::new(
        BigUint::new([
            1920338615, 978607672, 3210029420, 1426256477, 2186553912, 1509376480, 2343017368,
            1847409506, 4079005044, 2394015518, 3196781879, 2861025826,
        ]),
        BigUint::new([
            2431258207, 2051218812, 494829981, 174109134, 3052452032, 3923390739, 681186428,
            4176747965, 2459098153, 1570674879, 2519084143, 907533898,
        ]),
    ),
    n: BigUint::new([
        3435473267, 3974895978, 1219536762, 1478102450, 4097256927, 3345173889, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
    ]),
    _private: (),
};

pub const P521: Config<U640> = Config {
    p: BigUint::new([
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    a: BigUint::new([
        4294967292, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    b: BigUint::new([
        1800421120, 4014284756, 1026307313, 896786312, 1001504519, 374522045, 3967718267,
        1444493649, 2398161377, 3098839441, 2578650611, 2732225115, 3062186222, 2459574688,
        2384239135, 2503915873, 81, 0, 0, 0,
    ]),
    g: Point::new(
        BigUint::new([
            3269836134, 4185816625, 2238333595, 860402625, 2734663902, 4263362855, 4024916264,
            2706071159, 1800224186, 4163415904, 88061217, 2623832377, 597013570, 2654915430,
            67430861, 2240677559, 198, 0, 0, 0,
        ]),
        BigUint::new([
            2681300560, 2294191222, 2725429824, 893153414, 1068304225, 3310401793, 1593058880,
            2548986521, 658400812, 397393175, 1469793384, 2566210633, 746396633, 1552572340,
            2587607044, 959015544, 280, 0, 0, 0,
        ]),
    ),
    n: BigUint::new([
        2436391945, 3144660766, 2308720558, 1001769400, 4144604624, 2144076104, 3207566955,
        1367771011, 4294967290, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    _private: (),
};


#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::{format, string::String};
    use big_num::BigUint;
    use numeric::FromStrRadix;

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
}


