#![allow(dead_code, unused_variables)]

use super::math_helpers::mod_pow;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/*
*****************
* Field Element *
*****************
* */
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FieldElement {
    pub num: u64,
    pub prime: u64,
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> Self {
        assert!(num < prime, "Error: Value is out of range of field");
        Self { num, prime }
    }

    pub fn pow(&self, exp: i32) -> Self {
        let mut exp = exp;
        let prime_i32: i32 = self
            .prime
            .try_into()
            .expect("Error: pow function overflowed");
        while exp < 0 {
            exp += prime_i32 - 1;
        }
        let exp: u64 = exp.try_into().expect("Error: pow function overflowed");
        let result = mod_pow(self.num, exp, self.prime);

        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

/*
* NOTE: FieldElement numbers are u64. During arithmetic operations, they are converted
*       into i128 because the operation might temporarily overflow u64. Once the rem_euclid
*       is taken on self.prime, the result will always fit into u64 because self.prime
*       is a u64.
*
* */

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Error: FieldElements must be of the same order to add"
        );

        let result = (self.num as i128)
            .checked_add(rhs.num as i128)
            .expect("Error: overflow occurred on FieldElement addition");
        let result = result.rem_euclid(self.prime as i128);
        let result: u64 = result
            .try_into()
            .expect("Error: overflow occurred on FieldElement addition");

        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let neg: i128 = (self.num as i128)
            .checked_neg()
            .expect("Error: overflow occurred on FieldElement negation");
        let result = neg.rem_euclid(self.prime as i128);
        let result: u64 = result
            .try_into()
            .expect("Error: overflow occurred on FieldElement negation");
        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Error: FieldElements must be of the same order to subtract"
        );

        let result = (self.num as i128)
            .checked_sub(rhs.num as i128)
            .expect("Error: underflow occurred on FieldElement subtraction");
        let result = result.rem_euclid(self.prime as i128);
        let result: u64 = result
            .try_into()
            .expect("Error: underflow occurred on FieldElement subtraction");

        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Error: FieldElements must be of the same order to multiply"
        );

        let result = (self.num as i128)
            .checked_mul(rhs.num as i128)
            .expect("Error: overflow occurred on FieldElement multiplication");
        let result = result.rem_euclid(self.prime as i128);
        let result: u64 = result
            .try_into()
            .expect("Error: overflow occurred on FieldElement multiplication");
        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Error: FieldElements must be of the same order to divide"
        );

        let exp: i32 = (rhs.prime - 2)
            .try_into()
            .expect("Error: overflow occurred on FieldElement division");

        let result = (self.num * rhs.pow(exp).num) % self.prime;

        Self {
            num: result,
            prime: self.prime,
        }
    }
}

/*
**********************
* FieldElement Macro *
* ********************
* this is purely for convenience
* */
#[macro_export]
macro_rules! field_element {
    ($num:expr, $prime:expr) => {
        FieldElement::new(($num), ($prime))
    };
}

/*
*********
* Tests *
* *******
* */
#[cfg(test)] // only include this module when testing
mod tests {
    // Import the outer scope
    use super::*;

    #[test]
    fn test_equal() {
        let element1 = FieldElement::new(7, 19);
        let element2 = FieldElement::new(7, 19);
        assert_eq!(element1, element1);
        assert_eq!(element2, element2);
        assert_eq!(element1, element2);
        assert_eq!(element2, element1);
    }

    #[test]
    fn test_neg() {
        let element1 = FieldElement::new(9, 19);
        assert_eq!(-element1, FieldElement::new(10, 19));
    }

    #[test]
    fn test_add() {
        let element1 = FieldElement::new(11, 19);
        let element2 = FieldElement::new(17, 19);
        assert_eq!(element1 + element2, FieldElement::new(9, 19));
    }

    #[test]
    fn test_sub() {
        let element1 = FieldElement::new(6, 19);
        let element2 = FieldElement::new(13, 19);
        assert_eq!(element1 - element2, FieldElement::new(12, 19));
    }

    #[test]
    fn test_mul() {
        let element1 = FieldElement::new(8, 19);
        let element2 = FieldElement::new(17, 19);
        assert_eq!(element1 * element2, FieldElement::new(3, 19));
    }

    #[test]
    fn test_pow() {
        let element = FieldElement::new(7, 19);
        assert_eq!(element.pow(3), FieldElement::new(1, 19));

        let element = FieldElement::new(5, 19);
        assert_eq!(element.pow(-3), FieldElement::new(7, 19));

        let element = FieldElement::new(7, 19);
        assert_eq!(element.pow(-2), FieldElement::new(7, 19));
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 200, 13), 9);
        assert_eq!(mod_pow(10, 1000, 17), 16);
        assert_eq!(mod_pow(7, 0, 5), 1);
        assert_eq!(mod_pow(5, 3, 13), 8);
    }

    #[test]
    fn test_div() {
        let element1 = FieldElement::new(10, 19);
        let element2 = FieldElement::new(3, 19);
        assert_eq!(element1 / element2, FieldElement::new(16, 19));

        let element1 = FieldElement::new(12, 19);
        let element2 = FieldElement::new(8, 19);
        assert_eq!(element1 / element2, FieldElement::new(11, 19));
    }

    #[test]
    fn test_macro() {
        let element1 = field_element!(2, 10);
        let element2 = FieldElement::new(2, 10);
        assert_eq!(element1, element2);
    }

    #[test]
    #[should_panic]
    fn test_macro_fail() {
        let element = field_element!(20, 19);
    }
}
