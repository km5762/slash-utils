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

pub struct VerifyingIntermediateValues<T> {
    u: Option<(T, T)>,
    generated_point: Option<Point<T>>,
    valid: bool,
}

impl <T> Default for VerifyingIntermediateValues<T> {
    fn default() -> Self {
        Self { u: None, generated_point: None, valid: false }
    }
}

impl<T: Numeric> Ecdsa<T>
where
    <T as Widen>::Output: Widened<T>
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

    pub fn verify(&self, key: &Point<T>, hash: &T, signature: &(T, T)) -> VerifyingIntermediateValues<T> {
        let Config { p, a, b, g, n, .. } = &self.config;
        let mut intermediate_values = VerifyingIntermediateValues::default();

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

        let (r, s) = *signature;

        let s_inv = match ring.inv(s) {
            Some(inv) => inv,
            None => return intermediate_values,
        };

        let u1 = ring.mul(*hash, s_inv);
        let u2 = ring.mul(r, s_inv);
        intermediate_values.u = Some((u1, u2));

        let point1 = match curve.mul(g, u1) {
            Some(point) => point,
            None => return intermediate_values,
        };

        let point2 = match curve.mul(key, u2) {
            Some(point) => point,
            None => return intermediate_values,
        };

        let random_point = match curve.add(&point1, &point2) {
            Some(point) => point,
            None => return intermediate_values,
        };

        intermediate_values.generated_point = Some(random_point.clone());

        if r == random_point.x {
            intermediate_values.valid = true;
            intermediate_values
        } else {
            intermediate_values
        }
    }
}
