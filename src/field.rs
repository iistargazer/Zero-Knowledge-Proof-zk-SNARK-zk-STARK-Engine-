use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;

pub const MODULUS: u64 = 101;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    pub value: u64,
}

impl FieldElement {
    pub fn new(val: u64) -> Self {
        FieldElement { value: val % MODULUS }
    }

    pub fn zero() -> Self {
        FieldElement { value: 0 }
    }

    pub fn one() -> Self {
        FieldElement { value: 1 }
    }

    pub fn pow(self, exp: u64) -> Self {
        let mut res = Self::one();
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                res = res * base;
            }
            base = base * base;
            e /= 2;
        }
        res
    }

    pub fn inv(self) -> Self {
        assert!(self.value != 0, "Cannot invert zero in prime field");
        self.pow(MODULUS - 2)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        FieldElement::new(rng.gen::<u64>())
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FieldElement::new((self.value + rhs.value) % MODULUS)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        FieldElement::new((self.value + MODULUS - (rhs.value % MODULUS)) % MODULUS)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        FieldElement::new((self.value * rhs.value) % MODULUS)
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self {
        FieldElement::zero() - self
    }
}
