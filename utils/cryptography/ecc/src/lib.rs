#![no_std]

pub mod configs;
pub mod wasm_adapter;

extern crate alloc;

use configs::Config;
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
        let Config { p, a, b, g, n, .. } = &self.config;

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

        let point = match curve.mul(&g, *k) {
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

    pub fn verify(&self, key: &Point<T>, hash: &T, signature: &(T, T)) -> bool {
        let Config { p, a, b, g, n, .. } = &self.config;

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

        let (r, s) = *signature;

        let s_inv = match ring.inv(s) {
            Some(inv) => inv,
            None => return false,
        };

        let u1 = ring.mul(*hash, s_inv);
        let u2 = ring.mul(r, s_inv);

        let point1 = match curve.mul(g, u1) {
            Some(point) => point,
            None => return false,
        };

        let point2 = match curve.mul(key, u2) {
            Some(point) => point,
            None => return false,
        };

        let random_point = match curve.add(&point1, &point2) {
            Some(point) => point,
            None => return false,
        };

        if r == random_point.x {
            true
        } else {
            false
        }
    }
}
