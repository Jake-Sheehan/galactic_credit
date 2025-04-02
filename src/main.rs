#![allow(unused)]
mod ecc;
use Bounded::Finite;
use ecc::elliptic_curve::{Bounded, EcPoint};
use ecc::field_element::FieldElement;

fn main() {
    println!("Welcome to the Galactic Empire");
    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    let x = FieldElement::new(192, 223);
    let y = FieldElement::new(105, 223);
    let p1 = EcPoint::new(Finite(x), Finite(y), a, b);
    println!("p1: {}", p1);
}
