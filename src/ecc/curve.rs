#![allow(unused)]

use rug::{Complete, Integer};

use super::field::{FieldElement, Pow};

use core::f64;
use std::{
    fmt::{Display, write},
    ops::{Add, Div, Mul, Neg, Sub},
};

/*
*********
* Enums *
*********
* */
#[derive(Debug, Clone, PartialEq)]
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
#[derive(PartialEq, Debug, Clone)]
pub struct EcPoint {
    pub x: Bounded<FieldElement>,
    pub y: Bounded<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl EcPoint {
    pub fn new(
        x: Bounded<FieldElement>,
        y: Bounded<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        use Bounded::{Finite, Infinity};

        if let (Finite(x), Finite(y)) = (x, y) {
            if y.pow(2) != &(&(x.pow(3)) + &(&a * &x)) + &b {
                panic!("Elliptic Curve Error: {x}, {y} is not on curve.");
            }
            Self {
                x: Finite(x),
                y: Finite(y),
                a,
                b,
            }
        } else {
            Self {
                x: Infinity,
                y: Infinity,
                a,
                b,
            }
        }
    }
}

impl Display for EcPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})_a:{}_b:{}", self.x, self.y, self.a, self.b)
    }
}

impl Add for &EcPoint {
    type Output = EcPoint;

    fn add(self, rhs: Self) -> Self::Output {
        use Bounded::{Finite, Infinity};

        // Not on the same curve
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "Error: Elliptic curve addition failed because points are not on the same curve"
            );
        }

        match (&self.x, &self.y, &rhs.x, &rhs.y) {
            // Infinity + Infinity = Infinity
            (Infinity, Infinity, Infinity, Infinity) => EcPoint {
                x: Infinity,
                y: Infinity,
                a: self.a.clone(),
                b: self.b.clone(),
            },
            // Infinity + Point = Point
            (Infinity, Infinity, Finite(x2), Finite(y2)) => EcPoint {
                x: Finite(x2.clone()),
                y: Finite(y2.clone()),
                a: self.a.clone(),
                b: self.b.clone(),
            },
            // Point + Infinity = Point
            (Finite(x1), Finite(x2), Infinity, Infinity) => EcPoint {
                x: Finite(x1.clone()),
                y: Finite(x2.clone()),
                a: self.a.clone(),
                b: self.b.clone(),
            },
            // Point + -Point = Infinity
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 == x2 && y1 != y2 => EcPoint {
                x: Infinity,
                y: Infinity,
                a: self.a.clone(),
                b: self.b.clone(),
            },
            // x1 != x2 then use point addition
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 != x2 => {
                let s = &(y2 - y1) / &(x2 - x1);
                let x3 = &(&(s.pow(2)) - x1) - x2;
                let y3 = &(&s * &(x1 - &x3)) - y1;

                EcPoint {
                    x: Finite(x3),
                    y: Finite(y3),
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            }
            // Point + itself = find tangent line intersection
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2)) if x1 == x2 && y1 == y2 => {
                let s = &((&(x1.pow(2)).scale(3)) + &self.a) / &(y1.scale(2));
                let x3 = &(s.pow(2)) - &(x1.scale(2));
                let y3 = &(&s * &(x1 - &x3)) - y1;

                EcPoint {
                    x: Finite(x3),
                    y: Finite(y3),
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            }
            // Apex point of curve
            (Finite(x1), Finite(y1), Finite(x2), Finite(y2))
                if x1 == x2 && y1 == y2 && y1.is_zero() =>
            {
                EcPoint {
                    x: Infinity,
                    y: Infinity,
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            }
            // Addition not defined for any other conditions
            (_, _, _, _) => panic!("Error: elliptic curve addition not defined for this condition"),
        }
    }
}

impl Mul<i32> for EcPoint {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        use Bounded::{Finite, Infinity};
        let mut result = EcPoint::new(Infinity, Infinity, self.a.clone(), self.b.clone());
        let mut current = self.clone();
        let mut rhs = rhs;

        while rhs > 0 {
            if rhs & 1 == 1 {
                result = &result + &current;
            }
            current = &current + &current;
            rhs >>= 1;
        }

        result
    }
}

impl Mul<EcPoint> for i32 {
    type Output = EcPoint;

    fn mul(self, rhs: EcPoint) -> Self::Output {
        use Bounded::{Finite, Infinity};
        let mut result = EcPoint::new(Infinity, Infinity, rhs.a.clone(), rhs.b.clone());
        let mut current = rhs;
        let mut lhs = self;

        while lhs > 0 {
            if lhs & 1 == 1 {
                result = &result + &current;
            }
            current = &current + &current;
            lhs >>= 1;
        }

        result
    }
}

impl Mul<Integer> for &EcPoint {
    type Output = EcPoint;

    fn mul(self, rhs: Integer) -> Self::Output {
        use Bounded::{Finite, Infinity};
        let mut result = EcPoint::new(Infinity, Infinity, self.a.clone(), self.b.clone());
        let mut current = self.clone();
        let mut rhs = rhs.clone();

        while rhs.is_positive() {
            if (&rhs & 1u8).complete() == 1 {
                result = &result + &current;
            }
            current = &current + &current;
            rhs >>= 1;
        }

        result
    }
}

impl Mul<&EcPoint> for Integer {
    type Output = EcPoint;

    fn mul(self, rhs: &EcPoint) -> Self::Output {
        use Bounded::{Finite, Infinity};
        let mut result = EcPoint::new(Infinity, Infinity, rhs.a.clone(), rhs.b.clone());
        let mut current = rhs.clone();
        let mut lhs = self.clone();

        while lhs.is_positive() {
            if (&lhs & 1u8).complete() == 1 {
                result = &result + &current;
            }
            current = &current + &current;
            lhs >>= 1;
        }

        result
    }
}

/*
***************
* Point Macro *
***************
* this is purely for convenience
* */
#[macro_export]
macro_rules! ec_point {
    ($x:expr, $y:expr, $a:expr, $b:expr) => {
        EcPoint::new($x, $y, $a, $b)
    };
}

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
        let p1 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
    }

    #[test]
    fn test_infinity_point() {
        let p1 = EcPoint::new(
            Infinity,
            Infinity,
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
    }

    #[test]
    #[should_panic]
    fn test_point_not_on_curve() {
        let p1 = EcPoint::new(
            Finite(FieldElement::new(200, 223)),
            Finite(FieldElement::new(119, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
    }

    #[test]
    fn test_point_eq() {
        let p1 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        let p2 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        assert_eq!(p1, p2);
        assert_eq!(p1, p1);
        assert_eq!(p2, p1);
        assert_eq!(p2, p2);
    }

    #[test]
    fn test_point_ne() {
        let p1 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        let p2 = EcPoint::new(
            Finite(FieldElement::new(17, 223)),
            Finite(FieldElement::new(56, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_infinity_add() {
        let p1 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        let inf = EcPoint::new(
            Infinity,
            Infinity,
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        assert_eq!(&p1 + &inf, p1);
        assert_eq!(&inf + &p1, p1);
    }

    #[test]
    fn test_add() {
        let prime = 97;
        let a = FieldElement::new(2, prime);
        let b = FieldElement::new(3, prime);

        let x1 = FieldElement::new(3, prime);
        let y1 = FieldElement::new(6, prime);
        let p1 = EcPoint::new(Finite(x1), Finite(y1), a.clone(), b.clone());

        let x2 = FieldElement::new(80, prime);
        let y2 = FieldElement::new(10, prime);
        let p2 = EcPoint::new(Finite(x2), Finite(y2), a.clone(), b.clone());

        // P + Q = R
        let x3 = FieldElement::new(80, prime);
        let y3 = FieldElement::new(87, prime);
        let expected = EcPoint::new(Finite(x3), Finite(y3), a.clone(), b.clone());
        assert_eq!(&p1 + &p2, expected);
    }

    #[test]
    fn test_add_point_to_itself() {
        let p1 = EcPoint::new(
            Finite(FieldElement::new(192, 223)),
            Finite(FieldElement::new(105, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        let result = EcPoint::new(
            Finite(FieldElement::new(49, 223)),
            Finite(FieldElement::new(71, 223)),
            FieldElement::new(0, 223),
            FieldElement::new(7, 223),
        );
        assert_eq!(&p1 + &p1, result);
    }
}
