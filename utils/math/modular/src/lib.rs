#![no_std]

use core::{
    mem::swap,
    ops::{Mul, Rem},
};

use numeric::{CheckedAdd, CheckedMul, CheckedSub, Narrow, One, RemEuclid, Widen, Zero};

pub trait Numeric:
    core::ops::Div<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::Add<Output = Self>
    + One
    + Zero
    + RemEuclid
    + CheckedSub
    + CheckedAdd
    + CheckedMul
    + Copy
    + Sized
    + core::cmp::PartialOrd
{
}
impl<T> Numeric for T where
    T: core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Add<Output = Self>
        + One
        + Zero
        + RemEuclid
        + CheckedSub
        + CheckedAdd
        + CheckedMul
        + Copy
        + Sized
        + core::cmp::PartialOrd
{
}

pub trait Narrowed: Numeric + Widen {}
impl<T> Narrowed for T where T: Numeric + Widen {}

pub trait Widened<T>: Numeric + Narrow<Output = T> {}
impl<T, U> Widened<U> for T where T: Numeric + Narrow<Output = U> {}

pub struct Ring<T> {
    modulus: T,
}

impl<T: Narrowed> Ring<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(modulus: T) -> Self {
        Ring { modulus }
    }

    fn extended_euclidean(&self, a: T, b: T) -> (T, T, T) {
        let mut r0 = a.widen();
        let mut r1 = b.widen();
        let mut s0 = T::one().widen();
        let mut s1 = T::zero().widen();
        let mut t0 = T::zero().widen();
        let mut t1 = T::one().widen();
        let mut n = 0;
        while r1 > T::zero().widen() {
            let q = r0 / r1;
            r0 = if r0 > q * r1 {
                r0 - q * r1
            } else {
                q * r1 - r0
            };
            swap(&mut r0, &mut r1);
            s0 = s0 + q * s1;
            swap(&mut s0, &mut s1);
            t0 = t0 + q * t1;
            swap(&mut t0, &mut t1);
            n += 1;
        }

        if (n % 2) != 0 {
            s0 = b.widen() - s0
        } else {
            t0 = a.widen() - t0
        };

        (r0.narrow(), s0.narrow(), t0.narrow())
    }

    pub fn inv(&self, a: T) -> Option<T> {
        let (gcd, x, _) = self.extended_euclidean(a, self.modulus);

        if gcd != T::one() {
            return None;
        }

        let inv = (x.widen() % self.modulus.widen() + self.modulus.widen()) % self.modulus.widen();
        Some(inv.narrow())
    }

    pub fn add(&self, a: T, b: T) -> T {
        let sum = a.widen() + b.widen();
        (sum.rem_euclid(&self.modulus.widen())).narrow()
    }

    pub fn sub(&self, a: T, b: T) -> T {
        if a >= b {
            a.checked_sub(&b)
                .unwrap_or_else(|| (a % self.modulus) - (b % self.modulus))
                .rem_euclid(&self.modulus)
        } else {
            self.modulus
                - b.checked_sub(&a)
                    .unwrap_or_else(|| (b % self.modulus) - (a % self.modulus))
        }
    }

    pub fn mul(&self, a: T, b: T) -> T {
        let product = a.widen() * b.widen();
        (product.rem_euclid(&self.modulus.widen())).narrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extended_euclidean_common_cases() {
        let ring = Ring { modulus: 13 };

        assert_eq!(ring.extended_euclidean(101, 13), (1, 4, -31));
        assert_eq!(ring.extended_euclidean(123, 19), (1, -2, 13));
        assert_eq!(ring.extended_euclidean(25, 36), (1, 13, -9));
        assert_eq!(ring.extended_euclidean(69, 54), (3, -7, 9));
        assert_eq!(ring.extended_euclidean(55, 79), (1, 23, -16));
        assert_eq!(ring.extended_euclidean(33, 44), (11, -1, 1));
        assert_eq!(ring.extended_euclidean(50, 70), (10, 3, -2));
    }

    #[test]
    fn mod_inv_common_cases() {
        let ring = Ring { modulus: 13 };

        assert_eq!(ring.inv(2), Some(7));
        assert_eq!(ring.inv(4), Some(10));
        assert_eq!(ring.inv(5), Some(8));
        assert_eq!(ring.inv(7), Some(2));
        assert_eq!(ring.inv(10), Some(4));

        assert_eq!(ring.inv(13), None);
        assert_eq!(ring.inv(0), None);

        let ring = Ring { modulus: 26 };
        assert_eq!(ring.inv(7), Some(15));
        assert_eq!(ring.inv(12), None);
    }

    #[test]
    fn mod_inv_edge_cases() {
        let ring = Ring { modulus: 17 };

        assert_eq!(ring.inv(1), Some(1));
        assert_eq!(ring.inv(16), Some(16));

        let ring = Ring { modulus: 20 };

        assert_eq!(ring.inv(3), Some(7));
        assert_eq!(ring.inv(15), None);

        let ring = Ring { modulus: 8 };

        assert_eq!(ring.inv(1), Some(1));
        assert_eq!(ring.inv(3), Some(3));
        assert_eq!(ring.inv(5), Some(5));
    }

    #[test]
    fn addition_common_cases() {
        let ring = Ring { modulus: 7 };

        // Test cases for addition within modulus
        assert_eq!(ring.add(2, 3), 5);
        assert_eq!(ring.add(5, 6), 4);
        assert_eq!(ring.add(0, 0), 0);

        // Test cases for addition with overflow
        assert_eq!(ring.add(5, 5), 3);
        assert_eq!(ring.add(6, 6), 5);
    }

    #[test]
    fn addition_at_num_max() {
        let ring = Ring { modulus: 7 };

        assert_eq!(ring.add(i8::MIN, i8::MIN), 3);
        assert_eq!(ring.add(i8::MAX, i8::MAX), 2);
    }

    #[test]
    fn subtraction_common_cases() {
        let ring = Ring { modulus: 7 };

        // Test cases for subtraction within modulus
        assert_eq!(ring.sub(4, 2), 2);
        assert_eq!(ring.sub(6, 3), 3);
        assert_eq!(ring.sub(0, 0), 0);

        // Test cases for subtraction with underflow
        assert_eq!(ring.sub(1, 2), 6);
        assert_eq!(ring.sub(2, 4), 5);
    }

    #[test]
    fn subtraction_at_num_max() {
        let ring = Ring { modulus: 7 };

        assert_eq!(ring.sub(i32::MIN, i32::MAX), 4);
        assert_eq!(ring.sub(i32::MAX, i32::MIN), 3);
    }

    #[test]
    fn multiplication_common_cases() {
        let ring = Ring { modulus: 7 };

        // Test cases for multiplication within modulus
        assert_eq!(ring.mul(2, 3), 6);
        assert_eq!(ring.mul(4, 2), 1);
        assert_eq!(ring.mul(5, 3), 1);
        assert_eq!(ring.mul(0, 6), 0);

        // Test cases for multiplication with overflow
        assert_eq!(ring.mul(3, 3), 2);
        assert_eq!(ring.mul(6, 6), 1);
    }

    #[test]
    fn multiplication_edge_cases() {
        let ring = Ring { modulus: 7 };

        assert_eq!(ring.mul(0, 0), 0);
        assert_eq!(ring.mul(0, 1), 0);
        assert_eq!(ring.mul(1, 0), 0);

        let ring = Ring { modulus: 13 };

        assert_eq!(ring.mul(12, 12), 1);
        assert_eq!(ring.mul(6, 11), 1);
    }

    #[test]
    fn multiplication_at_num_max() {
        let ring = Ring { modulus: 7 };

        assert_eq!(ring.mul(i8::MAX, i8::MAX), 1);
        assert_eq!(ring.mul(i8::MAX, i8::MIN), 5);
    }
}
