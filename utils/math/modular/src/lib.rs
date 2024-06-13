#![no_std]

use num_traits::CheckedAdd;
use num_traits::CheckedMul;
use num_traits::CheckedSub;
use num_traits::Euclid;
use num_traits::One;
use num_traits::Zero;

struct Ring<T> {
    modulus: T,
}

impl<T> Ring<T>
where
    T: core::ops::Div<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Rem<Output = T>
        + core::ops::Add<Output = T>
        + One
        + Zero
        + Euclid
        + CheckedSub
        + CheckedAdd
        + CheckedMul
        + Copy
        + Sized
        + core::cmp::PartialOrd,
{
    fn extended_euclidean(a: T, b: T) -> (T, T, T) {
        let (mut x, mut y, mut x1, mut y1, mut a1, mut b1) =
            (T::one(), T::zero(), T::zero(), T::one(), a, b);

        while b1 > T::zero() {
            let q = a1 / b1;
            (x, x1) = (x1, x - q * x1);
            (y, y1) = (y1, y - q * y1);
            (a1, b1) = (b1, a1 - q * b1);
        }

        (a1, x, y)
    }

    fn mod_inv(&self, a: T) -> Option<T> {
        let (gcd, x, _) = Self::extended_euclidean(a, self.modulus);

        if gcd != T::one() {
            return None;
        }

        let inv = (x % self.modulus + self.modulus) % self.modulus;
        Some(inv)
    }

    fn add(&self, a: T, b: T) -> T {
        let sum = a
            .checked_add(&b)
            .unwrap_or_else(|| (a % self.modulus) + (b % self.modulus));
        sum.rem_euclid(&self.modulus)
    }

    fn sub(&self, a: T, b: T) -> T {
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

    fn mul(&self, a: T, b: T) -> T {
        let product = a
            .checked_mul(&b)
            .unwrap_or_else(|| (a % self.modulus) * (b % self.modulus));
        product.rem_euclid(&self.modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extended_euclidean_common_cases() {
        let extended_euclidean = Ring::extended_euclidean;

        assert_eq!(extended_euclidean(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean(25, 36), (1, 13, -9));
        assert_eq!(extended_euclidean(69, 54), (3, -7, 9));
        assert_eq!(extended_euclidean(55, 79), (1, 23, -16));
        assert_eq!(extended_euclidean(33, 44), (11, -1, 1));
        assert_eq!(extended_euclidean(50, 70), (10, 3, -2));
    }

    #[test]
    fn mod_inv_common_cases() {
        let ring = Ring { modulus: 13 };

        assert_eq!(ring.mod_inv(2), Some(7));
        assert_eq!(ring.mod_inv(4), Some(10));
        assert_eq!(ring.mod_inv(5), Some(8));
        assert_eq!(ring.mod_inv(7), Some(2));
        assert_eq!(ring.mod_inv(10), Some(4));

        assert_eq!(ring.mod_inv(13), None);
        assert_eq!(ring.mod_inv(0), None);

        let ring = Ring { modulus: 26 };
        assert_eq!(ring.mod_inv(7), Some(15));
        assert_eq!(ring.mod_inv(12), None);
    }

    #[test]
    fn mod_inv_edge_cases() {
        let ring = Ring { modulus: 17 };

        assert_eq!(ring.mod_inv(1), Some(1));
        assert_eq!(ring.mod_inv(16), Some(16));

        let ring = Ring { modulus: 20 };

        assert_eq!(ring.mod_inv(3), Some(7));
        assert_eq!(ring.mod_inv(15), None);

        let ring = Ring { modulus: 8 };

        assert_eq!(ring.mod_inv(1), Some(1));
        assert_eq!(ring.mod_inv(3), Some(3));
        assert_eq!(ring.mod_inv(5), Some(5));
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
