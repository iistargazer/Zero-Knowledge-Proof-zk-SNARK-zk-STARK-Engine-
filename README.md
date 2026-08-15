# Groth16 Zero-Knowledge Proof System in Rust

A complete from-scratch implementation of the Groth16 zk-SNARK protocol in Rust.

## Project Structure
- `src/field.rs`: Prime Field arithmetic modulo $p = 101$.
- `src/poly.rs`: Polynomial operations (addition, multiplication, evaluation, division, interpolation).
- `src/r1cs.rs`: Rank-1 Constraint System (R1CS) representation and verification.
- `src/qap.rs`: Quadratic Arithmetic Program (QAP) transformation using Lagrange Interpolation.
- `src/srs.rs`: Structured Reference String (SRS) generation and Trusted Setup.
- `src/main.rs`: End-to-end execution of Setup, Prove, and Verify.

## Usage
Run the project with Cargo:
```bash
cargo run
```
