use crate::field::FieldElement;
use crate::poly::Polynomial;
use crate::r1cs::R1CS;

#[derive(Debug)]
pub struct QAP {
    pub a_polys: Vec<Polynomial>,
    pub b_polys: Vec<Polynomial>,
    pub c_polys: Vec<Polynomial>,
    pub target_poly: Polynomial,
}

impl QAP {
    pub fn from_r1cs(r1cs: &R1CS) -> Self {
        let num_constraints = r1cs.a.len();
        let num_vars = r1cs.num_variables;
        let xs: Vec<FieldElement> = (1..=num_constraints as u64)
            .map(FieldElement::new)
            .collect();

        let mut a_polys = Vec::with_capacity(num_vars);
        let mut b_polys = Vec::with_capacity(num_vars);
        let mut c_polys = Vec::with_capacity(num_vars);

        for j in 0..num_vars {
            let ys_a: Vec<FieldElement> = (0..num_constraints).map(|i| r1cs.a[i][j]).collect();
            let ys_b: Vec<FieldElement> = (0..num_constraints).map(|i| r1cs.b[i][j]).collect();
            let ys_c: Vec<FieldElement> = (0..num_constraints).map(|i| r1cs.c[i][j]).collect();

            a_polys.push(Polynomial::interpolate(&xs, &ys_a));
            b_polys.push(Polynomial::interpolate(&xs, &ys_b));
            c_polys.push(Polynomial::interpolate(&xs, &ys_c));
        }

        let mut target_poly = Polynomial::new(vec![FieldElement::one()]);
        for &x in &xs {
            let term = Polynomial::new(vec![-x, FieldElement::one()]);
            target_poly = target_poly.mul(&term);
        }

        QAP { a_polys, b_polys, c_polys, target_poly }
    }

    pub fn compute_h(&self, witness: &[FieldElement]) -> Polynomial {
        let mut a_sum = Polynomial::zero();
        let mut b_sum = Polynomial::zero();
        let mut c_sum = Polynomial::zero();

        for i in 0..witness.len() {
            let w = witness[i];
            a_sum = a_sum.add(&self.a_polys[i].mul(&Polynomial::new(vec![w])));
            b_sum = b_sum.add(&self.b_polys[i].mul(&Polynomial::new(vec![w])));
            c_sum = c_sum.add(&self.c_polys[i].mul(&Polynomial::new(vec![w])));
        }

        let p = a_sum.mul(&b_sum).sub(&c_sum);
        let (h, rem) = p.div_rem(&self.target_poly);
        assert!(rem.is_zero(), "P(x) must be divisible by T(x)");
        h
    }
}
