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

pub struct Ecdsa<T> {
    config: Config<T>,
}

impl<T: Numeric> Ecdsa<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(config: Config<T>) -> Self {
        Self { config }
    }

    fn sign(&self, k: T, private_key: T, hash: T) -> Result<Signature<T>, Error> {
        let Config { p, a, b, g, n } = &self.config;

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

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

    // fn verify(&self, r: T, s: T, public_key: Point<T>, hash: T) -> bool {
    //     let Config { a, b, g, n } = &self.config;
    //     let curve = Curve::new(*a, *b, *n);
    //     let ring = Ring::new(*n);

    //     let s_inv = match ring.inv(s) {
    //         Some(inv) => inv,
    //         None => return false,
    //     };
    //     let u1 = ring.mul(hash, s_inv);
    //     let u2 = ring.mul(r, s_inv);

    //     let point1 = match curve.mul(&g, u1) {
    //         Some(point) => point,
    //         None => return false,
    //     };

    //     let point2 = match curve.mul(&public_key, u2) {
    //         Some(point) => point,
    //         None => return false,
    //     };

    //     let random_point = match curve.add(&point1, &point2) {
    //         Some(point) => point,
    //         None => return false,
    //     };

    //     r == random_point.x
    // }
}
