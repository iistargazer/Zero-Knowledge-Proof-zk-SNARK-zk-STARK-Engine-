use crate::field::FieldElement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub coeffs: Vec<FieldElement>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<FieldElement>) -> Self {
        let mut p = Polynomial { coeffs };
        p.trim();
        p
    }

    pub fn zero() -> Self {
        Polynomial { coeffs: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    pub fn degree(&self) -> usize {
        if self.is_zero() { 0 } else { self.coeffs.len() - 1 }
    }

    pub fn trim(&mut self) {
        while let Some(last) = self.coeffs.last() {
            if last.value == 0 {
                self.coeffs.pop();
            } else {
                break;
            }
        }
    }

    pub fn eval(&self, x: FieldElement) -> FieldElement {
        let mut result = FieldElement::zero();
        let mut x_pow = FieldElement::one();
        for &c in &self.coeffs {
            result = result + c * x_pow;
            x_pow = x_pow * x;
        }
        result
    }

    pub fn add(&self, rhs: &Polynomial) -> Polynomial {
        let max_len = std::cmp::max(self.coeffs.len(), rhs.coeffs.len());
        let mut result = vec![FieldElement::zero(); max_len];
        for i in 0..max_len {
            let c1 = if i < self.coeffs.len() { self.coeffs[i] } else { FieldElement::zero() };
            let c2 = if i < rhs.coeffs.len() { rhs.coeffs[i] } else { FieldElement::zero() };
            result[i] = c1 + c2;
        }
        Polynomial::new(result)
    }

    pub fn sub(&self, rhs: &Polynomial) -> Polynomial {
        let max_len = std::cmp::max(self.coeffs.len(), rhs.coeffs.len());
        let mut result = vec![FieldElement::zero(); max_len];
        for i in 0..max_len {
            let c1 = if i < self.coeffs.len() { self.coeffs[i] } else { FieldElement::zero() };
            let c2 = if i < rhs.coeffs.len() { rhs.coeffs[i] } else { FieldElement::zero() };
            result[i] = c1 - c2;
        }
        Polynomial::new(result)
    }

    pub fn mul(&self, rhs: &Polynomial) -> Polynomial {
        if self.is_zero() || rhs.is_zero() {
            return Polynomial::zero();
        }
        let mut result = vec![FieldElement::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in rhs.coeffs.iter().enumerate() {
                result[i + j] = result[i + j] + c1 * c2;
            }
        }
        Polynomial::new(result)
    }

    pub fn div_rem(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        assert!(!divisor.is_zero(), "Division by zero polynomial");
        let mut quotient = Polynomial::zero();
        let mut remainder = self.clone();

        while !remainder.is_zero() && remainder.degree() >= divisor.degree() {
            let deg_diff = remainder.degree() - divisor.degree();
            let lead_rem = *remainder.coeffs.last().unwrap();
            let lead_div = *divisor.coeffs.last().unwrap();
            let scale = lead_rem / lead_div;

            let mut term_coeffs = vec![FieldElement::zero(); deg_diff + 1];
            term_coeffs[deg_diff] = scale;
            let term_poly = Polynomial::new(term_coeffs);

            quotient = quotient.add(&term_poly);
            let sub_poly = divisor.mul(&term_poly);
            remainder = remainder.sub(&sub_poly);
        }

        (quotient, remainder)
    }

    pub fn interpolate(xs: &[FieldElement], ys: &[FieldElement]) -> Self {
        assert_eq!(xs.len(), ys.len());
        let n = xs.len();
        let mut result = Polynomial::zero();

        for i in 0..n {
            let mut l_i = Polynomial::new(vec![FieldElement::one()]);
            for j in 0..n {
                if i == j { continue; }
                let num = Polynomial::new(vec![-xs[j], FieldElement::one()]);
                let denom = xs[i] - xs[j];
                let scale = num.mul(&Polynomial::new(vec![denom.inv()]));
                l_i = l_i.mul(&scale);
            }
            result = result.add(&l_i.mul(&Polynomial::new(vec![ys[i]])));
        }
        result
    }
}
