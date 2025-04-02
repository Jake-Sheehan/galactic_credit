#![allow(unused)]

use super::field_element::FieldElement;
use super::math_helpers::{Pow, Scalable, Zero, u64_to_f64_safe};
use core::f64;
use std::{
    fmt::{Display, write},
    ops::{Add, Div, Mul, Neg, Sub},
    process::Output,
};

/*
**********
* Traits *
**********
* */
pub trait EcRepresentable:
    PartialEq
    + Display
    + Add<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Neg<Output = Self>
    + Pow
    + Scalable
    + Zero
    + Sized
    + Copy
    + Clone
{
}

impl Pow for f64 {
    fn pow(self, n: i32) -> Self {
        f64::powi(self, n)
    }
}

impl Zero for f64 {
    fn is_zero(&self) -> bool {
        *self == 0.0f64
    }
}

impl Scalable for f64 {
    fn scale(self, factor: u64) -> Self {
        let n: f64 = u64_to_f64_safe(factor);
        self * n
    }
}

impl EcRepresentable for f64 {}

impl EcRepresentable for FieldElement {}

/*
*********
* Enums *
*********
* */
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Bounded<T> {
    Finite(T),
    Infinity,
}

impl<T: Display> Display for Bounded<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Bounded::{Finite, Infinity};
        match self {
            Finite(val) => write!(f, "{val}"),
            Infinity => write!(f, "Infinity"),
        }
    }
}

/*
*********
* Point *
* *******
* */

// T must be able to be represented as a point on an EC
// x and y could be infinite, so we use Bounded<T>
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct EcPoint<T: EcRepresentable> {
    pub x: Bounded<T>,
    pub y: Bounded<T>,
    pub a: T,
    pub b: T,
}

impl<T: EcRepresentable> EcPoint<T> {
    pub fn new(x: Bounded<T>, y: Bounded<T>, a: T, b: T) -> Self {
        use Bounded::{Finite, Infinity};

        match (x, y) {
            (Infinity, Infinity) => Self { x, y, a, b },
            (Finite(x), Finite(y)) => {
                if y.pow(2) != x.pow(3) + a * x + b {
                    panic!("Elliptic Curve Error: {x}, {y} is not on curve.");
                }
                Self {
                    x: Finite(x),
                    y: Finite(y),
                    a,
                    b,
                }
            }
            (_, _) => {
                panic!(
                    "Error: cannot create EcPoint; cannot have one discrete value and one infinity"
                )
            }
        }
    }
}

impl<T: EcRepresentable> Display for EcPoint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})_a:{}_b:{}", self.x, self.y, self.a, self.b)
    }
}

impl<T: EcRepresentable> Add for EcPoint<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Bounded::{Finite, Infinity};

        // Not on the same curve
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "Error: Elliptic curve addition failed because points are not on the same curve"
            );
        }

        match (self.x, self.y, rhs.x, rhs.y) {
            // Infinity + Infinity = Infinity
            (Infinity, Infinity, Infinity, Infinity) => Self {
                x: Infinity,
                y: Infinity,
                a: self.a,
                b: self.b,
            },
            // Infinity + Point = Point
            (Infinity, Infinity, Finite(x2), Finite(y2)) => Self {
                x: Finite(x2),
                y: Finite(y2),
                a: self.a,
                b: self.b,
            },
            // Point + Infinity = Point
            (Finite(x1), Finite(x2), Infinity, Infinity) => Self {
                x: Finite(x1),
                y: Finite(x2),
                a: self.a,
                b: self.b,
            },
            // Point + -Point = Infinity
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 == x2 && y1 != y2 => Self {
                x: Infinity,
                y: Infinity,
                a: self.a,
                b: self.b,
            },
            // x1 != x2 then use point addition
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 != x2 => {
                let s = (y2 - y1) / (x2 - x1);
                let x3 = s.pow(2) - x1 - x2;
                let y3 = s * (x1 - x3) - y1;

                Self {
                    x: Finite(x3),
                    y: Finite(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            // Point + itself = find tangent line intersection
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 == x2 && y1 == y2 => {
                let s = ((x1.pow(2)).scale(3) + self.a) / (y1.scale(2));
                let x3 = s.pow(2) - x1.scale(2);
                let y3 = s * (x1 - x3) - y1;

                Self {
                    x: Finite(x3),
                    y: Finite(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            // Apex point of curve
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2))
                if x1 == x2 && y1 == y2 && y1.is_zero() =>
            {
                Self {
                    x: Infinity,
                    y: Infinity,
                    a: self.a,
                    b: self.b,
                }
            }
            // Addition not defined for any other conditions
            (_, _, _, _) => panic!("Error: elliptic curve addition not defined for this condition"),
        }
    }
}

/*
***************
* Point Macro *
***************
* this is purely for convenience
* */

/*
*********
* Tests *
*********
* */
#[cfg(test)]
mod tests {
    use Bounded::{Finite, Infinity};
    use core::f64;

    use super::*;

    #[test]
    fn test_point() {
        let point = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
    }

    #[test]
    fn test_infinity_point() {
        let point = EcPoint::new(Infinity, Infinity, 5.0, 7.0);
    }

    #[test]
    #[should_panic]
    fn test_point_not_on_curve() {
        let point = EcPoint::new(Finite(-1.0), Finite(-2.0), 5.0, 7.0);
    }

    #[test]
    fn test_point_eq() {
        let p1 = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        let p2 = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }

    #[test]
    fn test_point_ne() {
        let p1 = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        let p2 = EcPoint::new(Finite(-1.0), Finite(1.0), 5.0, 7.0);
        let p3 = EcPoint::new(Finite(2.0), Finite(5.0), 5.0, 7.0);
        let p4 = EcPoint::new(Finite(2.0), Finite(-5.0), 5.0, 7.0);
        let p5 = EcPoint::new(Finite(3.0), Finite(7.0), 5.0, 7.0);
        let p6 = EcPoint::new(Finite(3.0), Finite(-7.0), 5.0, 7.0);
        assert_ne!(p1, p2);
        assert_ne!(p3, p4);
        assert_ne!(p5, p6);
    }

    #[test]
    fn test_infinity_add() {
        let p1 = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        let p2 = EcPoint::new(Finite(-1.0), Finite(1.0), 5.0, 7.0);
        let inf = EcPoint::new(Infinity, Infinity, 5.0, 7.0);

        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p2, p2);
        assert_eq!(p1 + p2, inf);
    }

    #[test]
    fn test_add_where_x_ne() {
        let p1 = EcPoint::new(Finite(2.0), Finite(5.0), 5.0, 7.0);
        let p2 = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        assert_eq!(p1 + p2, EcPoint::new(Finite(3.0), Finite(-7.0), 5.0, 7.0));
    }

    #[test]
    fn test_add_point_to_itself() {
        let point = EcPoint::new(Finite(-1.0), Finite(-1.0), 5.0, 7.0);
        assert_eq!(
            point + point,
            EcPoint::new(Finite(18.0), Finite(77.0), 5.0, 7.0)
        );
    }
}
