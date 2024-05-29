#[derive(PartialEq, Debug)]
struct Point {
    x: i128,
    y: i128
}

impl Point {
    fn new (x: i128, y: i128) -> Point {
        Point {x, y}
    }
}

struct Curve {
    a: i128,
    b: i128
}

impl Curve {
    fn new (a: i128, b: i128) -> Curve {
        Curve {a, b}
    }

    fn add (self, p: &Point, q: &Point) -> Point {
        let slope: i128 = if p != q {
            (q.y - p.y) / (q.x- p.x)
        } else {
            (3 * ( p.x * p.x) + self.a ) / (2 * p.y)
        };

        let x = slope * slope - p.x - q.x;
        let y = slope * (p.x - x) - p.y;

        Point {
            x, y
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let p = Point::new(1, 2);
        let q = Point::new(3, 4);
        let curve = Curve::new(1, 1);
        assert_eq!(Point::new(-3 ,2), curve.add(&p, &q));
    }
}
