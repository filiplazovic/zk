pub mod elliptic;
pub mod qap;
pub mod r1cs;
pub mod merkle;

mod keccak;

use ark_ff::FftField;
use ark_poly::{
	polynomial::{univariate::DensePolynomial, UVPolynomial},
	EvaluationDomain,
};
use ark_std::test_rng;

fn test_fft<F: FftField, D: EvaluationDomain<F>>(degree: usize) {
	let mut rng = &mut test_rng();

	let mut a = DensePolynomial::<F>::rand(degree, &mut rng)
		.coeffs()
		.to_vec();

	let domain = D::new(degree).unwrap();
	domain.fft_in_place(&mut a);
}

fn main() {}