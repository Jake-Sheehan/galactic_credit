#![allow(dead_code, unused_variables)]

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// helpers
pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result: u64 = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }

    result
}

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
