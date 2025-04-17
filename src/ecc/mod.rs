pub mod curve;
pub mod field;
pub mod secp256k1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ec_point, field_element};
    use curve::{
        Bounded::{Finite, Infinity},
        EcPoint,
    };
    use field::FieldElement;

    #[test]
    fn test_finite_field_ec() {
        let a = field_element!(0, 223);
        let b = field_element!(7, 223);
        let fe1 = field_element!(192, 223);
        let fe2 = field_element!(105, 223);
        let p1 = EcPoint::new(Finite(fe1), Finite(fe2), a.clone(), b.clone());
        let fe3 = field_element!(17, 223);
        let fe4 = field_element!(56, 223);
        let p2 = EcPoint::new(Finite(fe3), Finite(fe4), a.clone(), b.clone());
        let fe5 = field_element!(1, 223);
        let fe6 = field_element!(193, 223);
        let p3 = EcPoint::new(Finite(fe5), Finite(fe6), a, b);
    }

    #[test]
    #[should_panic]
    fn test_finite_field_ec_fail() {
        let a = field_element!(0, 223);
        let b = field_element!(7, 223);
        let fe1 = field_element!(200, 223);
        let fe2 = field_element!(119, 223);
        let p1 = EcPoint::new(Finite(fe1), Finite(fe2), a, b);
    }

    #[test]
    fn test_finite_field_ec_addition() {
        let a = field_element!(0, 223);
        let b = field_element!(7, 223);

        let x1 = field_element!(170, 223);
        let y1 = field_element!(142, 223);
        let x2 = field_element!(60, 223);
        let y2 = field_element!(139, 223);
        let p1 = ec_point!(Finite(x1), Finite(y1), a.clone(), b.clone());
        let p2 = ec_point!(Finite(x2), Finite(y2), a.clone(), b.clone());
        let x3 = field_element!(220, 223);
        let y3 = field_element!(181, 223);
        let p3 = ec_point!(Finite(x3), Finite(y3), a.clone(), b.clone());
        assert_eq!(&p1 + &p2, p3);

        let x1 = field_element!(47, 223);
        let y1 = field_element!(71, 223);
        let x2 = field_element!(17, 223);
        let y2 = field_element!(56, 223);
        let p1 = ec_point!(Finite(x1), Finite(y1), a.clone(), b.clone());
        let p2 = ec_point!(Finite(x2), Finite(y2), a.clone(), b.clone());
        let x3 = field_element!(215, 223);
        let y3 = field_element!(68, 223);
        let p3 = ec_point!(Finite(x3), Finite(y3), a.clone(), b.clone());
        assert_eq!(&p1 + &p2, p3);

        let x1 = field_element!(143, 223);
        let y1 = field_element!(98, 223);
        let x2 = field_element!(76, 223);
        let y2 = field_element!(66, 223);
        let p1 = ec_point!(Finite(x1), Finite(y1), a.clone(), b.clone());
        let p2 = ec_point!(Finite(x2), Finite(y2), a.clone(), b.clone());
        let x3 = field_element!(47, 223);
        let y3 = field_element!(71, 223);
        let p3 = ec_point!(Finite(x3), Finite(y3), a, b);
        assert_eq!(&p1 + &p2, p3);
    }

    //     #[test]
    //     fn test_scalar_mul_on_finite_field_ec() {
    //         let prime = 223;
    //         let a = field_element!(0, prime);
    //         let b = field_element!(7, prime);
    //         let x = field_element!(15, prime);
    //         let y = field_element!(86, prime);
    //         let p = ec_point!(Finite(x), Finite(y), a.clone(), b.clone());
    //         let p2 = ec_point!(Infinity, Infinity, a, b);
    //         assert_eq!(&p * 7, p2);
    //         assert_eq!(7 * &p, p2);
    //     }
}
