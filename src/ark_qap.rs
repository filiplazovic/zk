use ark_ff::{FftField, PrimeField};
use ark_poly::{
	polynomial::{univariate::DensePolynomial, UVPolynomial},
	EvaluationDomain,
};
use ark_r1cs_std::poly::evaluations::univariate::lagrange_interpolator::LagrangeInterpolator;
use ark_std::test_rng;

fn test_fft<F: FftField, D: EvaluationDomain<F>>(degree: usize) {
	let mut rng = &mut test_rng();

	let mut a = DensePolynomial::<F>::rand(degree, &mut rng)
		.coeffs()
		.to_vec();

	let domain = D::new(degree).unwrap();
	domain.fft_in_place(&mut a);
}

pub fn run_example<F: PrimeField>() {
	let a = [
		[0, 1, 0, 0, 0, 0],
		[0, 0, 0, 1, 0, 0],
		[0, 1, 0, 0, 1, 0],
		[5, 0, 0, 0, 0, 1],
	];

	let b = [
		[0, 1, 0, 0, 0, 0],
		[0, 1, 0, 0, 0, 0],
		[1, 0, 0, 0, 0, 0],
		[1, 0, 0, 0, 0, 0],
	];

	let c = [
		[0, 0, 0, 1, 0, 0],
		[0, 0, 0, 0, 1, 0],
		[0, 0, 0, 0, 0, 1],
		[0, 0, 1, 0, 0, 0],
	];
}
