mod field_element;

fn main() {
    println!("Welcome to the Galactic Empire");
}

#[cfg(test)] // only include this module when testing
mod tests {
    // Import the outer scope
    use super::*;
    use field_element::{FieldElement, mod_pow};

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
}
