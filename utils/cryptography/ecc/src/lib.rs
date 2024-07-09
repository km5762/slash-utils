#![no_std]

pub mod configs;
pub mod errors;
pub mod wasm_adapter;

extern crate alloc;

use configs::Config;
use elliptic_curve::{Curve, Numeric, Point};
use errors::{InvalidGeneratorError, InvalidPointError, NoInvKError, SigningError, ZeroingKError};
use modular::{Ring, Widened};
use numeric::Widen;

pub struct Ecdsa<T> {
    config: Config<T>,
}

impl<T: Numeric> Ecdsa<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub const fn new(config: Config<T>) -> Self {
        Self { config }
    }

    pub fn sign(&self, k: &T, key: &T, hash: &T) -> Result<(T, T), SigningError> {
        let Config { p, a, b, g, n } = &self.config;

        let curve = Curve::new(*a, *b, *p);
        let ring = Ring::new(*n);

        if !curve.is_valid_point(&g) {
            return Err(errors::SigningError::InvalidGenerator(
                InvalidGeneratorError,
            ));
        }

        let point = match curve.mul(&g, *k) {
            Some(p) => p,
            None => return Err(errors::SigningError::InvalidPoint(InvalidPointError)),
        };

        let r = point.x;

        if r == T::zero() {
            return Err(errors::SigningError::ZeroingK(ZeroingKError));
        }

        let k_inv = match ring.inv(*k) {
            Some(inv) => inv,
            None => return Err(errors::SigningError::NoInvK(NoInvKError)),
        };

        let s = ring.mul(k_inv, ring.add(*hash, ring.mul(r, *key)));

        Ok((r, s))
    }

    pub fn verify(&self, key: &Point<T>, hash: &T, signature: &(T, T)) -> Result<(), ()> {
        let Config { p: _, a, b, g, n } = &self.config;
        let curve = Curve::new(*a, *b, *n);
        let ring = Ring::new(*n);

        let (r, s) = *signature;

        let s_inv = match ring.inv(s) {
            Some(inv) => inv,
            None => return Err(()),
        };

        let u1 = ring.mul(*hash, s_inv);
        let u2 = ring.mul(r, s_inv);

        let point1 = match curve.mul(g, u1) {
            Some(point) => point,
            None => return Err(()),
        };

        let point2 = match curve.mul(key, u2) {
            Some(point) => point,
            None => return Err(()),
        };

        let random_point = match curve.add(&point1, &point2) {
            Some(point) => point,
            None => return Err(()),
        };

        if r == random_point.x {
            Ok(())
        } else {
            Err(())
        }
    }
}
