use ark_ff::{FftField, PrimeField};
use ark_poly::{
	polynomial::{univariate::DensePolynomial, UVPolynomial},
	EvaluationDomain, Radix2EvaluationDomain, domain, Polynomial
};
use ark_bls12_381::Fr as Bls381;

pub fn evals_to_coeffs<F: PrimeField>() {
	let evals = vec![F::from(1000u32), F::from(2000u32), F::from(3000u32)];

	let d = Radix2EvaluationDomain::<F>::new(4).unwrap();

	let coeffs = d.ifft(&evals);

	let poly = DensePolynomial::<F>::from_coefficients_vec(coeffs);

	let eval1 = poly.evaluate(&F::from(0u32));
	let eval2 = poly.evaluate(&F::from(1u32));
	let eval3 = poly.evaluate(&F::from(2u32));


	println!("{}", eval1);
	println!("{}", eval2);
	println!("{}", eval3);
}

pub fn run_example() {
	evals_to_coeffs::<Bls381>();
}