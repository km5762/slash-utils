#![no_std]

use crypto_bigint::{AddMod, CtChoice, Invert, MulMod, SubMod};

trait BigInt<T>: AddMod<T, Output = T> + SubMod<T, Output = T> + MulMod<T, Output = T> + Invert + core::cmp::PartialEq {
    fn inv_mod(&self, modulus: &Self) -> (Self, CtChoice);
}

#[derive(Clone, PartialEq, Debug)]
struct Point<T> where T: BigInt<T> {
    x: T,
    y: T,
}

impl <T> Point<T> where T: BigInt<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point {x, y}
    }
}

struct Curve<T> where T: BigInt<T> {
    a: T,
    b: T,
    n: T,
}


impl <T> Curve<T> where T: BigInt<T> {
    fn new(a: T, b: T, n : T) -> Curve<T> {
        Curve {a, b, n}
    }

    fn add(&self, p: &Point<T>, q: &Point<T>) -> Option<Point<T>> {
        let slope = if p != q {
            let num = q.y.sub_mod(&p.y, &self.n);
            let inv = match q.x.sub_mod(&p.x, &self.n).inv_mod(&self.n) {
                (value, CtChoice::TRUE) => value,
                (_, CtChoice::FALSE) => return None,
            };
            
            
        } else {
            (3 * (p.x.mul_mod(p.x, p, p_inv)) + self.a ) / (2 * p.y)
        };



        let x = slope * slope - p.x - q.x;
        let y = slope * (p.x - x) - p.y;

        Point {
            x, y
        }
    }

    fn multiply(&self, p: &Point, d: u128) -> Point {
        let mut i = 128 - d.leading_zeros() as usize - 1;   
        let mut res = p.clone();

        while i > 0 {
            res = self.add(&res, &res);

            if ((d >> i) & 1) == 1 {
                res = self.add(&res, &p);
            }

            i -= 1;
        }

        res
    }

    fn compute_p_inv(p: T) {
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_p_not_equal_q() {
        let p = Point::new(1, 2);
        let q = Point::new(3, 4);
        let curve = Curve::new(-7, 10);
        assert_eq!(Point::new(-3 ,2), curve.add(&p, &q));
    }

    #[test]
    fn add_p_equals_q() {
        let p = Point::new(1, 2);
        let q = Point::new(1, 2);
        let curve = Curve::new(-7, 10);
        assert_eq!(Point::new(-1, -4), curve.add(&p, &q));
    }

    #[test]
    fn double() {
        let p = Point::new(3, 10);
        let curve = Curve::new(1, 1);
        assert_eq!(Point::new(80, 87), curve.multiply(&p, 2));
    }
}
