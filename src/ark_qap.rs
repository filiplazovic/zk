use ark_ff::{FftField, PrimeField};
use ark_poly::{
	polynomial::{univariate::DensePolynomial, UVPolynomial},
	EvaluationDomain, Radix2EvaluationDomain, domain, Polynomial
};
use ark_r1cs_std::poly::evaluations::univariate::lagrange_interpolator::LagrangeInterpolator;
use ark_std::test_rng;

pub fn run_example<F: PrimeField>() {
	// (0,3), (1,2), (2,4)
	let evals = vec![F::from(3u32), F::from(2u32), F::from(4u32)];

	let d = Radix2EvaluationDomain::<F>::new(2).unwrap();

	let coeffs = d.ifft(&evals);

	let poly = DensePolynomial::<F>::from_coefficients_vec(coeffs);

	let eval1 = poly.evaluate(&evals[0]);
	let eval2 = poly.evaluate(&evals[1]);
	let eval3 = poly.evaluate(&evals[2]);


	println!("{}", eval1);
	println!("{}", eval2);
	println!("{}", eval3);
}
