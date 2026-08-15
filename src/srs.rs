use crate::field::FieldElement;
use crate::poly::Polynomial;
use crate::qap::QAP;

#[derive(Debug)]
pub struct SRS {
    pub tau_powers: Vec<FieldElement>,
    pub alpha_tau_powers: Vec<FieldElement>,
    pub beta_tau_powers: Vec<FieldElement>,
    pub ht_powers: Vec<FieldElement>,
}

impl SRS {
    pub fn setup(qap: &QAP, tau: FieldElement, alpha: FieldElement, beta: FieldElement) -> Self {
        let max_degree = qap.target_poly.degree() + 1;

        let mut tau_powers = Vec::with_capacity(max_degree + 1);
        let mut alpha_tau_powers = Vec::with_capacity(max_degree + 1);
        let mut beta_tau_powers = Vec::with_capacity(max_degree + 1);

        let mut curr_tau = FieldElement::one();
        for _ in 0..=max_degree {
            tau_powers.push(curr_tau);
            alpha_tau_powers.push(alpha * curr_tau);
            beta_tau_powers.push(beta * curr_tau);
            curr_tau = curr_tau * tau;
        }

        let t_tau = qap.target_poly.eval(tau);
        let mut ht_powers = Vec::with_capacity(qap.target_poly.degree());
        let mut curr_t = FieldElement::one();
        for _ in 0..qap.target_poly.degree() {
            ht_powers.push(curr_t * t_tau);
            curr_t = curr_t * tau;
        }

        SRS {
            tau_powers,
            alpha_tau_powers,
            beta_tau_powers,
            ht_powers,
        }
    }
}

pub struct Proof {
    pub a: FieldElement,
    pub b: FieldElement,
    pub c: FieldElement,
}

pub fn prove(qap: &QAP, srs: &SRS, witness: &[FieldElement], r: FieldElement, s: FieldElement) -> Proof {
    let mut a_poly = Polynomial::zero();
    let mut b_poly = Polynomial::zero();
    let mut c_poly = Polynomial::zero();

    for i in 0..witness.len() {
        let w = witness[i];
        a_poly = a_poly.add(&qap.a_polys[i].mul(&Polynomial::new(vec![w])));
        b_poly = b_poly.add(&qap.b_polys[i].mul(&Polynomial::new(vec![w])));
        c_poly = c_poly.add(&qap.c_polys[i].mul(&Polynomial::new(vec![w])));
    }

    let h_poly = qap.compute_h(witness);

    let eval_poly = |poly: &Polynomial, powers: &[FieldElement]| -> FieldElement {
        let mut res = FieldElement::zero();
        for (i, &c) in poly.coeffs.iter().enumerate() {
            res = res + c * powers[i];
        }
        res
    };

    let a_eval = eval_poly(&a_poly, &srs.tau_powers) + r;
    let b_eval = eval_poly(&b_poly, &srs.tau_powers) + s;

    let h_eval = eval_poly(&h_poly, &srs.ht_powers);

    let c_eval = eval_poly(&c_poly, &srs.tau_powers) + h_eval + s * a_eval + r * b_eval - r * s;

    Proof { a: a_eval, b: b_eval, c: c_eval }
}

pub fn verify(proof: &Proof) -> bool {
    proof.a * proof.b == proof.c
}
