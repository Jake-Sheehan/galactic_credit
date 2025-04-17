use rug::ops::{RemRounding, RemRoundingAssign};
use rug::{Complete, Integer};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Pow {
    fn pow<T: Into<Integer>>(&self, exp: T) -> Self;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct FieldElement {
    pub value: Integer,
    pub order: Integer,
}

impl FieldElement {
    pub fn new<T, U>(value: T, order: U) -> Self
    where
        T: Into<Integer>,
        U: Into<Integer>,
    {
        let value: Integer = value.into();
        let order: Integer = order.into();
        assert!(
            !value.is_negative() && value <= order,
            "Error: value out of range, cannot create FieldElement"
        );
        Self { value, order }
    }

    pub fn from_str(value: &str, order: &str) -> Self {
        let value: Integer = Integer::from_str_radix(value, 16).unwrap();
        let order: Integer = Integer::from_str_radix(order, 16).unwrap();
        assert!(
            !value.is_negative() && value <= order,
            "Error: value out of range, cannot create FieldElement"
        );
        Self { value, order }
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn scale(&self, scalar: i32) -> Self {
        let mut result = Integer::from(&self.value * scalar);
        result.rem_euc_assign(&self.order);
        Self {
            value: result,
            order: self.order.clone(),
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.order, self.value)
    }
}

impl Neg for &FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        let mut neg = (-&self.value).complete();
        neg.rem_euc_assign(&self.order);

        FieldElement {
            value: neg,
            order: self.order.clone(),
        }
    }
}

impl Add<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: &FieldElement) -> Self::Output {
        assert!(
            self.order == rhs.order,
            "Error: add operation failed because FieldElements are not the same order"
        );
        let mut result = (&self.value + &rhs.value).complete();
        result.rem_euc_assign(&self.order);

        FieldElement {
            value: result,
            order: self.order.clone(),
        }
    }
}

impl Sub<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: &FieldElement) -> Self::Output {
        assert!(
            self.order == rhs.order,
            "Error: sub operation failed because FieldElements are not the same order"
        );

        let mut result = (&self.value - &rhs.value).complete();
        result.rem_euc_assign(&self.order);

        FieldElement {
            value: result,
            order: self.order.clone(),
        }
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &FieldElement) -> Self::Output {
        assert!(
            self.order == rhs.order,
            "Error: mul operation failed because FieldElements are not the same order"
        );

        let mut result = (&self.value * &rhs.value).complete();
        result.rem_euc_assign(&self.order);

        FieldElement {
            value: result,
            order: self.order.clone(),
        }
    }
}

impl Div<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &FieldElement) -> Self::Output {
        assert!(
            self.order == rhs.order,
            "Error: div operation failed because FieldElements are not the same order"
        );

        let exp = (&self.order - 2i32).complete();
        let result = rhs
            .value
            .pow_mod_ref(&exp, &self.order)
            .expect("Error: FieldElement div failed")
            .complete();
        let mut result = (&self.value * &result).complete();
        result.rem_euc_assign(&self.order);

        FieldElement {
            value: result,
            order: self.order.clone(),
        }
    }
}

impl Pow for FieldElement {
    fn pow<T: Into<Integer>>(&self, exp: T) -> Self {
        let mut exp: Integer = exp.into();
        let result = (&self.order - 1i32).complete();
        exp.rem_euc_assign(&result);
        let result = self
            .value
            .pow_mod_ref(&exp, &self.order)
            .expect("Error: FieldElement exponent failed")
            .complete();

        FieldElement {
            value: result,
            order: self.order.clone(),
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
        FieldElement::new($num, $prime)
    };
}

/*********
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
        assert_eq!(-&element1, FieldElement::new(10, 19));
    }

    #[test]
    fn test_add() {
        let element1 = FieldElement::new(11, 19);
        let element2 = FieldElement::new(17, 19);
        assert_eq!(&element1 + &element2, FieldElement::new(9, 19));
    }

    #[test]
    fn test_sub() {
        let element1 = FieldElement::new(6, 19);
        let element2 = FieldElement::new(13, 19);
        assert_eq!(&element1 - &element2, FieldElement::new(12, 19));
    }

    #[test]
    fn test_mul() {
        let element1 = FieldElement::new(8, 19);
        let element2 = FieldElement::new(17, 19);
        assert_eq!(&element1 * &element2, FieldElement::new(3, 19));
    }

    #[test]
    fn test_pow() {
        let element = FieldElement::new(7, 19);
        assert_eq!(element.pow(&Integer::from(3)), FieldElement::new(1, 19));

        let element = FieldElement::new(5, 19);
        assert_eq!(element.pow(&Integer::from(-3)), FieldElement::new(7, 19));

        let element = FieldElement::new(7, 19);
        assert_eq!(element.pow(&Integer::from(-2)), FieldElement::new(7, 19));
    }

    #[test]
    fn test_div() {
        let element1 = FieldElement::new(10, 19);
        let element2 = FieldElement::new(3, 19);
        assert_eq!(&element1 / &element2, FieldElement::new(16, 19));

        let element1 = FieldElement::new(12, 19);
        let element2 = FieldElement::new(8, 19);
        assert_eq!(&element1 / &element2, FieldElement::new(11, 19));
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
