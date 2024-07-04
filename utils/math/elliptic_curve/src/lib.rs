#![no_std]

use core::ops::{BitAnd, Shr};
use modular::{Narrowed, Ring, Widened};
use numeric::{Bit, LeadingZeros, Widen};

pub trait Numeric: modular::Narrowed + From<u8> + Bit + LeadingZeros {}

impl<T> Numeric for T where T: modular::Narrowed + From<u8> + Bit + LeadingZeros {}

#[derive(Clone, PartialEq, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

pub struct Curve<T> {
    a: T,
    b: T,
    ring: Ring<T>,
}

impl<T: Numeric> Curve<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(a: T, b: T, modulus: T) -> Curve<T> {
        Curve {
            a,
            b,
            ring: Ring::new(modulus),
        }
    }

    pub fn add(&self, p: &Point<T>, q: &Point<T>) -> Option<Point<T>> {
        let three = T::from(3);
        let two = T::from(2);

        let slope = if p != q {
            let num = self.ring.sub(q.y, p.y);
            let denom = self.ring.sub(q.x, p.x);

            match self.ring.inv(denom) {
                Some(inv) => self.ring.mul(num, inv),
                None => return None,
            }
        } else {
            let px2 = self.ring.mul(p.x, p.x);
            let num = self.ring.add(self.ring.mul(three, px2), self.a);
            let denom = self.ring.mul(two, p.y);

            match self.ring.inv(denom) {
                Some(inv) => self.ring.mul(num, inv),
                None => return None,
            }
        };

        let slope2 = self.ring.mul(slope, slope);
        let x = self.ring.sub(self.ring.sub(slope2, p.x), q.x);
        let y = self
            .ring
            .sub(self.ring.mul(slope, self.ring.sub(p.x, x)), p.y);

        Some(Point::new(x, y))
    }

    pub fn mul(&self, p: &Point<T>, d: T) -> Option<Point<T>> {
        let size = core::mem::size_of::<T>() * 8;
        let bits = size - d.leading_zeros() as usize;
        let mut res = p.clone();

        for i in (0..bits - 1).rev() {
            res = match self.add(&res, &res) {
                Some(res) => res,
                None => return None,
            };

            if d.bit(i) {
                res = match self.add(&res, &p) {
                    Some(res) => res,
                    None => return None,
                };
            }
        }

        Some(res)
    }

    pub fn is_valid_point(&self, p: &Point<T>) -> bool {
        let lhs = self.ring.mul(p.y, p.y);
        let rhs = self.ring.add(
            self.ring.add(
                self.ring.mul(self.ring.mul(p.x, p.x), p.x),
                self.ring.mul(self.a, p.x),
            ),
            self.b,
        );
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_p_not_equal_q() {
        let p = Point::new(12, 4);
        let q = Point::new(11, 4);
        let curve = Curve::new(-7, 10, 13);
        assert_eq!(Some(Point::new(3, 9)), curve.add(&p, &q));
    }

    #[test]
    fn add_p_equals_q() {
        let p = Point::new(12, 4);
        let q = Point::new(12, 4);
        let curve = Curve::new(-7, 10, 13);
        assert_eq!(Some(Point::new(12, 9)), curve.add(&p, &q));
    }

    #[test]
    fn point_not_on_curve() {
        let curve = Curve::new(-7, 10, 13);
        assert!(!curve.is_valid_point(&Point::new(10, 3)));
        assert!(!curve.is_valid_point(&Point::new(14, 3)));
    }

    #[test]
    fn point_on_curve() {
        let curve = Curve::new(-7, 10, 13);

        let valid_points = [
            Point::new(0, 6),
            Point::new(0, 7),
            Point::new(1, 11),
            Point::new(2, 11),
            Point::new(3, 9),
            Point::new(5, 10),
            Point::new(11, 9),
        ];

        for point in valid_points {
            assert!(
                curve.is_valid_point(&point),
                "Point ({}, {}) should be on the curve but was reported as invalid",
                point.x,
                point.y
            );
        }
    }

    #[test]
    fn mul_double() {
        let p = Point::new(11, 4);
        let curve = Curve::new(-7, 10, 13);
        assert_eq!(Some(Point::new(5, 3)), curve.mul(&p, 2));
    }

    #[test]
    fn mul_triple() {
        let p = Point::new(11, 4);
        let curve = Curve::new(-7, 10, 13);
        assert_eq!(Some(Point::new(1, 2)), curve.mul(&p, 3));
    }
}
