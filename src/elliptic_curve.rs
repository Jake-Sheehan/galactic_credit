#![allow(unused)]

use core::f64;
use std::{
    fmt::{Display, write},
    ops::Add,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub b: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, a: f64, b: f64) -> Self {
        if x.is_infinite() && y.is_infinite() {
            return Self { x, y, a, b };
        }
        if y.powi(2) != x.powi(3) + a * x + b {
            panic!("Elliptic Curve Error: {x}, {y} is not on curve.");
        }
        Self { x, y, a, b }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "Error: Elliptic curve addition failed because points are not on the same curve"
            );
        }

        // Point + INFINITY = Point
        if self.x == f64::INFINITY {
            return rhs;
        }

        // Point + INFINITY = Point
        if rhs.x == f64::INFINITY {
            return self;
        }

        // Point + -Point = INFINITY
        if self.x == rhs.x && self.y != rhs.y {
            return Self {
                x: f64::INFINITY,
                y: f64::INFINITY,
                a: self.a,
                b: self.b,
            };
        }

        // x1 != x2; use standard point addition
        if self.x != rhs.x {
            let s = (rhs.y - self.y) / (rhs.x - self.x);
            let x = s.powi(2) - self.x - rhs.x;
            let y = s * (self.x - x) - self.y;

            return Self {
                x,
                y,
                a: self.a,
                b: self.b,
            };
        }

        // Point + itself = find tangent line intersection
        if self == rhs {
            let s = (3.0 * self.x.powi(2) + self.a) / (2.0 * self.y);
            let x = (s.powi(2)) - (2.0 * self.x);
            let y = s * (self.x - x) - self.y;

            return Self {
                x,
                y,
                a: self.a,
                b: self.b,
            };
        }

        if self == rhs && self.y == 0.0 {
            return Self {
                x: f64::INFINITY,
                y: f64::INFINITY,
                a: self.a,
                b: self.b,
            };
        }

        panic!("Error: elliptic curve addition failed all conditions");
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;

    #[test]
    fn test_point() {
        let point = Point::new(-1.0, -1.0, 5.0, 7.0);
    }

    #[test]
    fn test_infinity_point() {
        let point = Point::new(f64::INFINITY, f64::INFINITY, 5.0, 7.0);
    }

    #[test]
    #[should_panic]
    fn test_point_not_on_curve() {
        let point = Point::new(-1.0, -2.0, 5.0, 7.0);
    }

    #[test]
    fn test_point_eq() {
        let p1 = Point::new(-1.0, -1.0, 5.0, 7.0);
        let p2 = Point::new(-1.0, -1.0, 5.0, 7.0);
        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }

    #[test]
    fn test_point_ne() {
        let p1 = Point::new(-1.0, -1.0, 5.0, 7.0);
        let p2 = Point::new(-1.0, 1.0, 5.0, 7.0);
        let p3 = Point::new(2.0, 5.0, 5.0, 7.0);
        let p4 = Point::new(2.0, -5.0, 5.0, 7.0);
        let p5 = Point::new(3.0, 7.0, 5.0, 7.0);
        let p6 = Point::new(3.0, -7.0, 5.0, 7.0);
        assert_ne!(p1, p2);
        assert_ne!(p3, p4);
        assert_ne!(p5, p6);
    }

    #[test]
    fn test_infinity_add() {
        let p1 = Point::new(-1.0, -1.0, 5.0, 7.0);
        let p2 = Point::new(-1.0, 1.0, 5.0, 7.0);
        let inf = Point::new(f64::INFINITY, f64::INFINITY, 5.0, 7.0);

        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p2, p2);
        assert_eq!(p1 + p2, inf);
    }

    #[test]
    fn add_where_x_ne() {
        let p1 = Point::new(2.0, 5.0, 5.0, 7.0);
        let p2 = Point::new(-1.0, -1.0, 5.0, 7.0);
        assert_eq!(p1 + p2, Point::new(3.0, -7.0, 5.0, 7.0));
    }

    #[test]
    fn add_point_to_itself() {
        let point = Point::new(-1.0, -1.0, 5.0, 7.0);
        assert_eq!(point + point, Point::new(18.0, 77.0, 5.0, 7.0));
    }
}
