use ark_ff::PrimeField;
use ark_std::test_rng;

pub fn hash<F: PrimeField>(x: F) -> F {
	let g = F::from(100u32);
	g.pow(x.into_repr())
}

pub fn test<F: PrimeField>() {
	let mut rng = test_rng();

	let secret = F::rand(&mut rng);
	let y = hash(secret);

	let v = F::rand(&mut rng);
	let t = hash(v);

	let c = F::rand(&mut rng);

	let r = v - c * secret;

	let t_calc = hash(r) * y.pow(c.into_repr());

	println!("{} {}", t, t_calc);
}
