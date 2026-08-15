mod field;
mod poly;
mod r1cs;
mod qap;
mod srs;

use field::FieldElement;
use r1cs::R1CS;
use qap::QAP;
use srs::{SRS, prove, verify};

fn main() {
    println!("=== Groth16 Zero-Knowledge Proof System ===");

    // Statement: Prover knows x such that x^3 + x + 5 = 35 (over GF(101))
    // Witness vector: [1, x, x^2, x^3, x^3 + x, x^3 + x + 5]
    // Variable values: [1, 3, 9, 27, 30, 35]

    let w = vec![
        FieldElement::new(1),
        FieldElement::new(3),
        FieldElement::new(9),
        FieldElement::new(27),
        FieldElement::new(30),
        FieldElement::new(35),
    ];

    let a = vec![
        vec![FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(5), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(1), FieldElement::new(0)],
    ];

    let b = vec![
        vec![FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
    ];

    let c = vec![
        vec![FieldElement::new(0), FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(1), FieldElement::new(0), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(1), FieldElement::new(0)],
        vec![FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(0), FieldElement::new(1)],
    ];

    let r1cs = R1CS::new(a, b, c, 1);
    assert!(r1cs.is_satisfied(&w), "R1CS must be satisfied by witness");
    println!("✓ R1CS satisfied by witness");

    let qap = QAP::from_r1cs(&r1cs);
    println!("✓ QAP transformed successfully");

    let tau = FieldElement::random();
    let alpha = FieldElement::random();
    let beta = FieldElement::random();

    let srs = SRS::setup(&qap, tau, alpha, beta);
    println!("✓ SRS (Trusted Setup) generated");

    let r = FieldElement::random();
    let s = FieldElement::random();

    let proof = prove(&qap, &srs, &w, r, s);
    println!("✓ Proof generated");

    let is_valid = verify(&proof);
    println!("✓ Verification result: {}", is_valid);
}
