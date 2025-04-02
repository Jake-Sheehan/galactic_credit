// Traits
pub trait Pow: Sized {
    fn pow(self, n: i32) -> Self;
}

pub trait Scalable: Sized {
    fn scale(self, factor: u64) -> Self;
}

pub trait Zero: Sized {
    fn is_zero(&self) -> bool;
}

// Functions
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

pub fn u64_to_f64_safe(value: u64) -> f64 {
    let float_value = value as f64;
    if float_value as u64 != value {
        panic!("Error: overflow occured trying to convert u64 to f64");
    }
    float_value
}

pub fn f64_to_u64_safe(value: f64) -> u64 {
    if value.is_nan() || value.is_infinite() || value < 0.0 || value > u64::MAX as f64 {
        panic!("Error: overflow occurred trying to convert f64 to u64");
    }
    let unsigned_value = value as u64;
    if unsigned_value as f64 != value {
        panic!("Error: overflow occurred trying to convert f64 to u64");
    }
    unsigned_value
}
