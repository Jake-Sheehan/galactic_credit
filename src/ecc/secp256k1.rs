use super::curve::{
    Bounded::{self, Finite, Infinity},
    EcPoint,
};
use super::field::FieldElement;
use rug::Integer;

const PRIME_STR: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
const A_STR: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const B_STR: &str = "0000000000000000000000000000000000000000000000000000000000000007";
const GX_STR: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const GY_STR: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
const N_STR: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

pub struct S256Field {
    element: FieldElement,
}

impl S256Field {
    pub fn new(element: &str) -> Self {
        Self {
            element: FieldElement::from_str(element, PRIME_STR),
        }
    }
}

pub struct S256Point {
    point: EcPoint,
}

impl S256Point {
    pub fn new(x: Bounded<S256Field>, y: Bounded<S256Field>) -> Self {
        let a = S256Field::new(A_STR);
        let b = S256Field::new(B_STR);

        let mut p: EcPoint;
        if let (Finite(x), Finite(y)) = (x, y) {
            p = EcPoint::new(Finite(x.element), Finite(y.element), a.element, b.element);
        } else {
            p = EcPoint::new(Infinity, Infinity, a.element, b.element);
        }
        Self { point: p }
    }

    pub fn get_generator() -> Self {
        let a = S256Field::new(A_STR);
        let b = S256Field::new(B_STR);
        let x = S256Field::new(GX_STR);
        let y = S256Field::new(GY_STR);
        let p = EcPoint::new(Finite(x.element), Finite(y.element), a.element, b.element);
        Self { point: p }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let g = S256Point::get_generator();
        let n: Integer = Integer::from_str_radix(N_STR, 16).unwrap();

        assert_eq!(
            n * &g.point,
            EcPoint::new(Infinity, Infinity, g.point.a, g.point.b)
        );
    }
}
